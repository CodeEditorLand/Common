//! # ProxyTarget DTO
//!
//! Defines the `ProxyTarget` enum for identifying the target service or
//! context for an RPC message, mirroring the `MainContext`/`ExtHostContext`
//! pattern from VS Code.

/// An enum that provides strongly-typed identifiers for all services that can
/// be communicated with across the IPC boundary.
///
/// This is used to construct fully qualified RPC method names, ensuring that a
/// message sent from one process is routed to the correct service
/// implementation in the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProxyTarget {
	// --- For ExtHost -> MainThread calls (Cocoon -> Mountain) ---
	MainThreadCommands,

	MainThreadConfiguration,

	MainThreadDiagnostics,

	MainThreadDocuments,

	MainThreadExtensionEnablement,

	MainThreadFileSystem,

	MainThreadLanguageFeatures,

	MainThreadLanguages,

	MainThreadOutputService,

	MainThreadSecrets,

	MainThreadStorage,

	MainThreadTerminalService,

	MainThreadWindow,

	MainThreadWebViews,

	MainThreadTelemetry,

	MainThreadWorkSpace,

	MainThreadStatusBar,

	MainThreadSourceControlManagement,

	MainThreadTesting,

	MainThreadDebugService,

	MainThreadTaskService,

	MainThreadCustomEditors,

	MainThreadTreeView,

	// --- For MainThread -> ExtHost calls (Mountain -> Cocoon) ---
	ExtHostCommands,

	ExtHostConfiguration,

	ExtHostDiagnostics,

	ExtHostDocuments,

	ExtHostExtensionService,

	ExtHostFileSystemInfo,

	ExtHostLanguageFeatures,

	ExtHostLanguages,

	ExtHostOutputService,

	ExtHostStorage,

	ExtHostTerminalService,

	ExtHostEnvironment,

	ExtHostWebViews,

	ExtHostTelemetry,

	ExtHostChatProvider,

	ExtHostExtensionEnablement,

	ExtHostCustomEditors,

	ExtHostQuickInput,

	ExtHostMessageService,

	ExtHostDialogs,

	ExtHostAuthentication,

	ExtHostDebugService,

	ExtHostTaskService,

	ExtHostManagedSockets,

	ExtHostTreeView,

	ExtHostSourceControlManagement,

	ExtHostTesting,
}

impl ProxyTarget {
	/// Returns a string prefix representing the target, used in constructing
	/// fully qualified RPC method names (e.g.,

	/// `MainThreadCommands$ExecuteCommand`).
	pub fn GetTargetPrefix(&self) -> String { format!("{:?}", self) }
}
