use async_trait::async_trait;
use serde_json::Value;

/// @module CommandExecutor
/// @description Defines the abstract service trait for command management
/// capabilities.
use crate::environment::Environment;
use crate::error::CommonError;

/// An abstract service contract for an environment component that can execute
/// and manage commands within the application.
///
/// This trait is implemented by `MountainEnvironment` and provides the core
/// logic for the command palette and programmatic command execution. It can
/// handle both native commands implemented in Rust and proxied commands
/// implemented in sidecars.
#[async_trait]
pub trait CommandExecutor: Environment + Send + Sync {
	/// Executes a command with the given ID and arguments.
	///
	/// @param CommandIdentifier - The unique ID of the command to execute.
	/// @param Argument - A `serde_json::Value` containing the arguments for
	/// the command. @returns A `Result` containing the command's return value
	/// as a `serde_json::Value`.
	async fn ExecuteCommand(&self, CommandIdentifier:String, Argument:Value) -> Result<Value, CommonError>;

	/// Registers a command that is implemented in an external sidecar process.
	///
	/// @param SidecarIdentifier - The ID of the sidecar where the command logic
	/// resides. @param CommandIdentifier - The unique ID of the command being
	/// registered.
	async fn RegisterCommand(&self, SidecarIdentifier:String, CommandIdentifier:String) -> Result<(), CommonError>;

	/// Unregisters a previously registered command.
	async fn UnregisterCommand(&self, SidecarIdentifier:String, CommandIdentifier:String) -> Result<(), CommonError>;

	/// Retrieves a list of all currently registered command IDs.
	async fn GetAllCommands(&self) -> Result<Vec<String>, CommonError>;
}
