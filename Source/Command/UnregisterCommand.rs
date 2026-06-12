//! # UnregisterCommand Effect
//!
//! Defines the `ActionEffect` for unregistering a previously registered
//! command.

use std::sync::Arc;

use super::CommandExecutor::CommandExecutor;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will unregister a command from the
/// host's command registry.
/// Typically called when an extension is deactivated or explicitly disposes of
/// a command registration.
///
/// # Parameters
/// * `SideCarIdentifier`: The unique ID of the sidecar that originally
///   registered the command.
/// * `CommandIdentifier`: The unique ID of the command to unregister.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn UnregisterCommand(
	SideCarIdentifier:String,

	CommandIdentifier:String,
) -> ActionEffect<Arc<dyn CommandExecutor>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Executor:Arc<dyn CommandExecutor>| {
		let SideCarIdentifierClone = SideCarIdentifier.clone();

		let CommandIdentifierClone = CommandIdentifier.clone();

		Box::pin(async move { Executor.UnregisterCommand(SideCarIdentifierClone, CommandIdentifierClone).await })
	}))
}
