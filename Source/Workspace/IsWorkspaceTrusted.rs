//! # IsWorkspaceTrusted Effect
//!
//! Defines the `ActionEffect` for checking if the current workspace is trusted.

use std::sync::Arc;

use super::WorkspaceProvider::WorkspaceProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will check if the current workspace
/// is considered trusted by the user.
///
/// Workspace Trust is a security feature that restricts certain operations
/// (like automatic task execution) in untrusted folders.
///
/// It uses the `WorkspaceProvider` capability from the environment.
///
/// # Returns
/// An `ActionEffect` that resolves with a `bool` indicating the trust state.
pub fn IsWorkspaceTrusted() -> ActionEffect<Arc<dyn WorkspaceProvider>, CommonError, bool> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn WorkspaceProvider>| {
		Box::pin(async move { Provider.IsWorkspaceTrusted().await })
	}))
}
