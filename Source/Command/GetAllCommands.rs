//! # GetAllCommands Effect
//!
//! Defines the `ActionEffect` for retrieving all registered command
//! identifiers.

use std::sync::Arc;

use super::CommandExecutor::CommandExecutor;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will retrieve a list of all currently
/// registered command identifiers.
///
/// This includes both native commands implemented in Rust and proxied commands
/// contributed by external sidecars. It uses the `CommandExecutor` capability
/// from the environment to perform the operation.
///
/// # Returns
///
/// An `ActionEffect` that resolves with a `Vec<String>` of command
/// identifiers. The capability required to run this effect is an
/// `Arc<dyn CommandExecutor>`.
pub fn GetAllCommands() -> ActionEffect<Arc<dyn CommandExecutor>, CommonError, Vec<String>> {
	ActionEffect::New(Arc::new(move |Executor:Arc<dyn CommandExecutor>| {
		Box::pin(async move { Executor.GetAllCommands().await })
	}))
}
