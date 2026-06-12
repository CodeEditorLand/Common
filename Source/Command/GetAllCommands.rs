// File: Common/Source/Command/GetAllCommands.rs
// Role: Defines the `GetAllCommands` ActionEffect.
// Responsibilities:
//   - Provide a declarative effect for retrieving a list of all registered
//     command identifiers.
//   - This effect abstracts the "what" (get all commands) from the "how" (the
//     CommandExecutor implementation).

//! # GetAllCommands Effect
//!
//! Defines a declarative `ActionEffect` for retrieving all currently registered
//! command identifiers from the `CommandExecutor`.

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
