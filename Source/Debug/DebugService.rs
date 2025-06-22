// File: Common/Source/Debug/DebugService.rs
// Role: Defines the abstract service trait for launching and managing debug
// sessions. Responsibilities:
//   - Provide a contract for registering debug configuration providers and
//     adapter factories.
//   - Provide a contract for starting debug sessions and controlling them
//     (e.g., sending commands).

//! # DebugService Trait
//!
//! Defines the abstract service trait for launching and managing debug
//! sessions.

use async_trait::async_trait;
use serde_json::Value;
use url::Url;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// the entire debug lifecycle, from configuration resolution to session
/// control.
#[async_trait]
pub trait DebugService: Environment + Send + Sync {
	/// Registers a provider for resolving debug configurations from an
	/// extension.
	///
	/// # Parameters
	/// * `DebugType`: The type of debugger this provider is for (e.g., "node").
	/// * `ProviderHandle`: A unique handle assigned by the extension host for
	///   this provider.
	/// * `SidecarIdentifier`: The identifier of the sidecar (e.g.,

	///   "cocoon-main") hosting the provider.
	async fn RegisterDebugConfigurationProvider(
		&self,

		DebugType:String,

		ProviderHandle:u32,

		SidecarIdentifier:String,
	) -> Result<(), CommonError>;

	/// Registers a factory for creating debug adapter descriptors from an
	/// extension.
	///
	/// # Parameters
	/// * `DebugType`: The type of debugger this factory is for.
	/// * `FactoryHandle`: A unique handle assigned by the extension host for
	///   this factory.
	/// * `SidecarIdentifier`: The identifier of the sidecar hosting the
	///   factory.
	async fn RegisterDebugAdapterDescriptorFactory(
		&self,

		DebugType:String,

		FactoryHandle:u32,

		SidecarIdentifier:String,
	) -> Result<(), CommonError>;

	/// Starts a new debugging session based on a launch configuration.
	///
	/// # Parameters
	/// * `FolderURI`: The URI of the workspace folder context for this debug
	///   session.
	/// * `Configuration`: The `DebugConfiguration` DTO to use for launching.
	///
	/// # Returns
	/// A `Result` containing a unique session ID string on success.
	async fn StartDebugging(&self, FolderURI:Option<Url>, Configuration:Value) -> Result<String, CommonError>;

	/// Sends a command to a running debug session. This corresponds to the
	/// Debug Adapter Protocol (DAP).
	///
	/// # Parameters
	/// * `SessionID`: The unique ID of the target debug session.
	/// * `Command`: The DAP command to send (e.g., "continue", "stepOver").
	/// * `Arguments`: A JSON value containing the arguments for the command.
	///
	/// # Returns
	/// A `Result` containing the JSON response from the debug adapter.
	async fn SendCommand(&self, SessionID:String, Command:String, Arguments:Value) -> Result<Value, CommonError>;
}
