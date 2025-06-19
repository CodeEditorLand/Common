//! # UnregisterCommand Effect
//!
//! Defines the `ActionEffect` for unregistering a previously registered
//! command.

use std::sync::Arc;

use super::CommandExecutor::CommandExecutor;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will unregister a command from the
/// host's command registry.
///
/// This is typically called when an extension is deactivated or explicitly
/// disposes of a command registration.
///
/// # Parameters
///
/// * `SidecarIdentifier`: The unique ID of the sidecar that originally
///   registered the command.
/// * `CommandIdentifier`: The unique ID of the command to unregister.
///
/// # Returns
///
/// An `ActionEffect` that resolves to `()` on success.
pub fn UnregisterCommand<TRunTime>(
	SidecarIdentifier:String,
	CommandIdentifier:String,
) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn CommandExecutor>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let SidecarIdentifierClone = SidecarIdentifier.clone();
		let CommandIdentifierClone = CommandIdentifier.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Executor:Arc<dyn CommandExecutor> = Environment.Require();
			Executor.UnregisterCommand(SidecarIdentifierClone, CommandIdentifierClone).await
		})
	}))
}
