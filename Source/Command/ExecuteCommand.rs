//! # ExecuteCommand Effect
//!
//! Defines the `ActionEffect` for executing a registered command.

use std::sync::Arc;

use serde_json::Value;

use super::CommandExecutor::CommandExecutor;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will run a command by its unique
/// identifier.
///
/// It uses the `CommandExecutor` capability from the environment to dispatch
/// the command to the appropriate handler, whether that handler is a native

/// Rust function or a proxied function in an external sidecar process.
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
pub fn ExecuteCommand<TRunTime>(
	CommandIdentifier:String,
	Argument:Value,
) -> ActionEffect<Arc<TRunTime>, CommonError, Value>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn CommandExecutor>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let CommandIdentifierClone = CommandIdentifier.clone();
		let ArgumentClone = Argument.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Executor:Arc<dyn CommandExecutor> = Environment.Require();
			Executor.ExecuteCommand(CommandIdentifierClone, ArgumentClone).await
		})
	}))
}
