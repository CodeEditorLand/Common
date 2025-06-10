use std::sync::Arc;

/// @module GetAllCommands
/// @description Defines the ActionEffect for retrieving all registered command
/// IDs.
use super::CommandExecutor::CommandExecutor;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will retrieve a list of all currently
/// registered command identifiers, including both native and proxied commands.
///
/// It uses the `CommandExecutor` capability from the environment to perform the
/// operation.
///
/// @returns An `ActionEffect` that resolves with a `Vec<String>` of command
/// IDs.
pub fn GetAllCommands<Runtime>() -> ActionEffect<Arc<Runtime>, CommonError, Vec<String>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn CommandExecutor>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Executor:Arc<dyn CommandExecutor> = Environment.Require();
			Executor.GetAllCommands().await
		})
	}))
}
