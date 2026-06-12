//! # ExecuteCommand Effect
//!
//! Defines the `ActionEffect` for executing a registered command.

use std::sync::Arc;

use serde_json::Value;

use super::CommandExecutor::CommandExecutor;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will run a command by its unique
/// identifier.
/// Uses the `CommandExecutor` capability from the environment to dispatch the
/// command to the appropriate handler, whether that handler is a native Rust
/// function or a proxied function in an external sidecar process.
///
/// # Parameters
///
/// * `CommandIdentifier`: The unique ID of the command to execute (e.g.,

///   "FileSystem.ReadFile").
/// * `Argument`: A `serde_json::Value` containing the arguments for the
///   command.
///
/// # Returns
///
/// An `ActionEffect` that resolves with the command's return value as a
/// `serde_json::Value`, or a `CommonError` if the command is not found or fails
/// during execution.
pub fn ExecuteCommand(
	CommandIdentifier:String,

	Argument:Value,
) -> ActionEffect<Arc<dyn CommandExecutor>, CommonError, Value> {
	ActionEffect::New(Arc::new(move |Executor:Arc<dyn CommandExecutor>| {
		let CommandIdentifierClone = CommandIdentifier.clone();

		let ArgumentClone = Argument.clone();

		Box::pin(async move { Executor.ExecuteCommand(CommandIdentifierClone, ArgumentClone).await })
	}))
}
