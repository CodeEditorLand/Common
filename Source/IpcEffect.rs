// Land_Common/src/ipc_effects.rs

use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;

use crate::environment::{Environment, Requires}; // For trait bounds on IpcProvider and in effects
use crate::runtime::AppRuntime; // Assumed concrete runtime accessor for effects
use crate::{effect::ActionEffect, errors::CommonError};

/// Identifies the target service or context for an IPC message, mirroring
/// the MainContext/ExtHostContext pattern in VS Code.
///
/// This enum helps in routing messages correctly when a central IPC mechanism
/// (like Vine) handles communication between Mountain (MainThread) and
/// various sidecars/extensions (ExtHosts).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProxyTarget {
	// --- For ExtHost -> MainThread calls (Sidecar -> Mountain) ---
	MainThreadCommands,
	MainThreadConfiguration,
	MainThreadDiagnostics,
	MainThreadDocuments,
	MainThreadExtensionEnablement, // For managing extension states
	MainThreadFileSystem,          // For vscode.workspace.fs operations
	MainThreadLanguageFeatures,    // For registering providers, etc.
	MainThreadLanguages,           // For language-related queries (e.g., getLanguages)
	MainThreadOutputService,
	MainThreadSecrets,
	MainThreadStorage,
	MainThreadTerminalService,
	MainThreadWindow,   // For window-specific actions (e.g., openExternal)
	MainThreadWebviews, // For webview panel management
	MainThreadTelemetry,
	MainThreadWorkspace, // For workspace info, findFiles
	MainThreadStatusBar, // For managing status bar items

	// --- For MainThread -> ExtHost calls (Mountain -> Sidecar) ---
	ExtHostCommands,         // To invoke commands registered by an extension
	ExtHostConfiguration,    // To notify extensions of config changes
	ExtHostDiagnostics,      // To push diagnostic collections to extensions (less common)
	ExtHostDocuments,        // To notify extensions of document changes
	ExtHostExtensionService, // For activation events (e.g., $activateByEvent)
	ExtHostFileSystemInfo,   // For file system provider info (e.g., $acceptProviderInfos)
	ExtHostLanguageFeatures, // To invoke language features implemented by extensions
	ExtHostLanguages,        // If MainThread pushes language info
	ExtHostOutputService,    // If MainThread manages log levels for extension output
	ExtHostStorage,          // To notify extensions of Memento changes (e.g., $acceptValue)
	ExtHostTerminalService,  // For terminal events (e.g., $acceptTerminalClosed)
	ExtHostEnv,              // For environment-related settings (e.g., telemetry level)
	ExtHostWebviews,         // For webview-related events/messages
	ExtHostTelemetry,        /* If MainThread pushes telemetry settings
	                          * ... other ExtHost services can be added as needed ... */
}

impl ProxyTarget {
	/// Returns a string prefix representing the target, often used in
	/// constructing fully qualified RPC method names.
	///
	/// Example: `ProxyTarget::MainThreadCommands.target_prefix()` might return
	/// "MainThreadCommands". This should align with the service naming
	/// convention in the specific IPC protocol being used (e.g., matching
	/// service names in VS Code's `extHost.protocol.ts`).
	pub fn target_prefix(&self) -> String { format!("{:?}", self) }
}

/// Trait defining operations for Inter-Process Communication (IPC).
///
/// An environment implementing this trait can send notifications and requests
/// to different sidecars (extensions or other processes).
#[async_trait]
pub trait IpcProvider: Environment {
	/// Sends a notification (fire-and-forget message) to a specified sidecar.
	async fn send_notification_to_sidecar(
		&self,
		sidecar_id:String,
		method:String,
		params:Value,
	) -> Result<(), CommonError>;

	/// Sends a request to a specified sidecar and awaits a response.
	async fn send_request_to_sidecar(
		&self,
		sidecar_id:String,
		method:String,
		params:Value,
		timeout_ms:u64,
	) -> Result<Value, CommonError>;

	// TODO: Consider adding methods for Mountain to send messages to Sky (frontend)
	// if that also goes through an abstracted IPC mechanism, though typically
	// that's done via direct Tauri `emit_all` or specific frontend-backend command
	// invocation.
}

// --- Effect Constructors ---

/// Creates an effect to send a notification to a sidecar.
pub fn send_notification_to_sidecar(
	sidecar_id:String,
	method:String,
	params:Value,
) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let sid_clone = sidecar_id.clone();
		let method_clone = method.clone();
		let params_clone = params.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let ipc_provider:Arc<dyn IpcProvider + Send + Sync> = concrete_env.require();
			ipc_provider
				.send_notification_to_sidecar(sid_clone, method_clone, params_clone)
				.await
		})
	}))
}

/// Creates an effect to send a request to a sidecar and await its response.
pub fn send_request_to_sidecar(
	sidecar_id:String,
	method:String,
	params:Value,
	timeout_ms:u64,
) -> ActionEffect<Arc<AppRuntime>, CommonError, Value> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let sid_clone = sidecar_id.clone();
		let method_clone = method.clone();
		let params_clone = params.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let ipc_provider:Arc<dyn IpcProvider + Send + Sync> = concrete_env.require();
			ipc_provider
				.send_request_to_sidecar(sid_clone, method_clone, params_clone, timeout_ms)
				.await
		})
	}))
}

/// Creates an effect that might be used to establish or confirm a connection
/// to a sidecar.
///
/// Currently, this is a placeholder that sends a simple internal notification.
/// A more robust implementation might involve specific handshake protocols.
pub fn establish_host_connection(sidecar_id:String) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Expects Arc<AppRuntime>
	// This could be a specific handshake message or a simple ping.
	// For now, reuses send_notification_to_sidecar.
	send_notification_to_sidecar(sidecar_id, "internal_ping_handshake".to_string(), Value::Null)
}

/// Creates an effect to proxy an RPC call (originally from one sidecar to
/// Mountain) to another target sidecar.
///
/// This is useful if Mountain acts as a router for certain cross-sidecar
/// communications.
///
/// # Argument
/// * `target_sidecar_id` - The ID of the sidecar to which the call should be
///   proxied.
/// * `call_data` - A JSON Value expected to contain `{"method": "methodName",
///   "params": ...}`.
pub fn proxy_call_to_sidecar(
	target_sidecar_id:String,
	call_data:Value, // Expected: { "method": string, "params": Value }
) -> ActionEffect<Arc<AppRuntime>, CommonError, Value> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let target_sid_clone = target_sidecar_id.clone();
		let call_data_clone = call_data.clone();
		Box::pin(async move {
			let method_str = call_data_clone
				.get("method")
				.and_then(Value::as_str)
				.ok_or_else(|| {
					CommonError::InvalidArg(
						"call_data.method".to_string(),
						"Expected a 'method' string field in call_data for proxying.".to_string(),
					)
				})?
				.to_string();

			let params_val = call_data_clone.get("params").cloned().unwrap_or(Value::Null);

			let concrete_env = app_runtime_accessor.get_environment();
			let ipc_provider:Arc<dyn IpcProvider + Send + Sync> = concrete_env.require();

			// Using a default timeout here; could be configurable or part of call_data.
			let default_timeout_ms = 30000;
			ipc_provider
				.send_request_to_sidecar(target_sid_clone, method_str, params_val, default_timeout_ms)
				.await
		})
	}))
}
