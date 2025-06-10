use std::sync::Arc;

/// @module UnregisterCommand
/// @description Defines the ActionEffect for unregistering a command.
use super::CommandExecutor::CommandExecutor;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will unregister a command from the
/// host's command registry.
///
/// This is typically called when an extension is deactivated or disposes of a
/// command registration.
///
/// @param SidecarIdentifier - The unique ID of the sidecar that originally
/// registered the command. @param CommandIdentifier - The unique ID of the
/// command to unregister.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn UnregisterCommand<Runtime>(
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
			Executor.UnregisterCommand(SidecarIdentifierClone, CommandIdentifierClone).await
		})
	}))
}
