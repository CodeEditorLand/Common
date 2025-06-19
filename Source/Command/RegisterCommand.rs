//! # RegisterCommand Effect
//!
//! Defines the `ActionEffect` for registering a command that is implemented in
//! an external sidecar process.

use std::sync::Arc;

use super::CommandExecutor::CommandExecutor;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

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
pub fn RegisterCommand(
	SidecarIdentifier:String,
	CommandIdentifier:String,
) -> ActionEffect<Arc<dyn CommandExecutor>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Executor:Arc<dyn CommandExecutor>| {
		let SidecarIdentifierClone = SidecarIdentifier.clone();
		let CommandIdentifierClone = CommandIdentifier.clone();
		Box::pin(async move { Executor.RegisterCommand(SidecarIdentifierClone, CommandIdentifierClone).await })
	}))
}
