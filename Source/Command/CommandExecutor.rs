//! # CommandExecutor Trait
//!
//! Defines the abstract service trait for command management and execution
//! capabilities.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can execute
/// and manage commands within the application.
///
/// This trait is implemented by the concrete `MountainEnvironment` and provides
/// the core logic for the command palette and programmatic command execution.
/// It is designed to handle both native commands implemented in Rust and
/// proxied commands implemented in external sidecars.
#[async_trait]
pub trait CommandExecutor: Environment + Send + Sync {
	/// Executes a command with the given identifier and arguments.
	///
	/// # Parameters
	///
	/// * `CommandIdentifier`: The unique ID of the command to execute.
	/// * `Argument`: A `serde_json::Value` containing the arguments for the
	///   command.
	///
	/// # Returns
	///
	/// A `Result` containing the command's return value as a
	/// `serde_json::Value` on success, or a `CommonError` on failure.
	async fn ExecuteCommand(&self, CommandIdentifier:String, Argument:Value) -> Result<Value, CommonError>;

	/// Registers a command that is implemented in an external sidecar process.
	///
	/// # Parameters
	///
	/// * `SideCarIdentifier`: The unique ID of the sidecar where the command
	///   logic resides.
	/// * `CommandIdentifier`: The unique ID of the command being registered.
	async fn RegisterCommand(&self, SideCarIdentifier:String, CommandIdentifier:String) -> Result<(), CommonError>;

	/// Unregisters a previously registered command.
	async fn UnregisterCommand(&self, SideCarIdentifier:String, CommandIdentifier:String) -> Result<(), CommonError>;

	/// Retrieves a list of all currently registered command identifiers.
	async fn GetAllCommands(&self) -> Result<Vec<String>, CommonError>;
}
