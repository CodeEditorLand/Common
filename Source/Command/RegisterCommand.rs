//! # RegisterCommand Effect
//!
//! Defines the `ActionEffect` for registering a command that is implemented in
//! an external sidecar process.

use std::sync::Arc;

use super::CommandExecutor::CommandExecutor;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will register a command that is
/// implemented in a sidecar process like Cocoon.
///
/// This allows the host application (`Mountain`) to know about commands
/// contributed by extensions so they can be displayed in the command palette
/// and invoked correctly. The `CommandExecutor` implementation will typically
/// store this as a `Proxied` command handler.
///
/// # Parameters
///
/// * `SidecarIdentifier`: The unique ID of the sidecar where the command logic
///   resides.
/// * `CommandIdentifier`: The unique ID of the command itself (e.g.,
///   "MyExtension.DoSomething").
///
/// # Returns
///
/// An `ActionEffect` that resolves to `()` on success.
pub fn RegisterCommand<TRunTime>(
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
			Executor.RegisterCommand(SidecarIdentifierClone, CommandIdentifierClone).await
		})
	}))
}
