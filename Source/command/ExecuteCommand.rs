use std::sync::Arc;

use serde_json::Value;

/// @module ExecuteCommand
/// @description Defines the ActionEffect for executing a command.
use super::CommandExecutor::CommandExecutor;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will run a command by its unique ID.
///
/// It uses the `CommandExecutor` capability from the environment to dispatch
/// the command to the appropriate handler, whether native or proxied.
///
/// @param CommandIdentifier - The unique ID of the command to execute.
/// @param Argument - A `serde_json::Value` containing the arguments for the
/// command.
///
/// @returns An `ActionEffect` that resolves with the command's return value as
/// a `serde_json::Value`.
pub fn ExecuteCommand<Runtime>(
	CommandIdentifier:String,
	Argument:Value,
) -> ActionEffect<Arc<Runtime>, CommonError, Value>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn CommandExecutor>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let CommandIdentifierClone = CommandIdentifier.clone();
		let ArgumentClone = Argument.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Executor:Arc<dyn CommandExecutor> = Environment.Require();
			Executor.ExecuteCommand(CommandIdentifierClone, ArgumentClone).await
		})
	}))
}
