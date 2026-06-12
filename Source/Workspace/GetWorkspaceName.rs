//! # GetWorkspaceName Effect
//!
//! Defines the `ActionEffect` for retrieving the name of the current
//! workspace.

use std::sync::Arc;

use super::WorkspaceProvider::WorkspaceProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will retrieve the display name of the
/// current workspace.
/// It uses the `WorkspaceProvider` capability from the environment to perform
/// the operation.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<String>` containing the
/// workspace name.
pub fn GetWorkspaceName() -> ActionEffect<Arc<dyn WorkspaceProvider>, CommonError, Option<String>> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn WorkspaceProvider>| {
		Box::pin(async move { Provider.GetWorkspaceName().await })
	}))
}
