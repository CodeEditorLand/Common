use std::sync::Arc;

/// @module RegisterCommand
/// @description Defines the ActionEffect for registering a command that is
/// implemented in an external sidecar process.
use super::CommandExecutor::CommandExecutor;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will register a command that is
/// implemented in a sidecar process like Cocoon.
///
/// This allows the host application (`Mountain`) to know about commands
/// contributed by extensions so they can be displayed in the command palette
/// and invoked correctly. The `CommandExecutor` implementation will store this
/// as a `Proxied` command.
///
/// @param SidecarIdentifier - The unique ID of the sidecar where the command
/// logic resides. @param CommandIdentifier - The unique ID of the command
/// itself (e.g., "myExtension.doSomething").
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn RegisterCommand<Runtime>(
	SidecarIdentifier:String,
	CommandIdentifier:String,
) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn CommandExecutor>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let SidecarIdentifierClone = SidecarIdentifier.clone();
		let CommandIdentifierClone = CommandIdentifier.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Executor:Arc<dyn CommandExecutor> = Environment.Require();
			Executor.RegisterCommand(SidecarIdentifierClone, CommandIdentifierClone).await
		})
	}))
}
