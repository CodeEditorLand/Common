//! # GetWorkspaceConfigurationPath Effect
//!
//! Defines the `ActionEffect` for retrieving the path to the workspace's
//! configuration file.

use std::{path::PathBuf, sync::Arc};

use super::WorkspaceProvider::WorkspaceProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will retrieve the file path of the
/// current workspace's configuration file (e.g., the `.code-workspace` file).
/// It uses the `WorkspaceProvider` capability from the environment.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<PathBuf>`, containing the
/// path if a workspace configuration file is open, or `None` otherwise.
pub fn GetWorkspaceConfigurationPath() -> ActionEffect<Arc<dyn WorkspaceProvider>, CommonError, Option<PathBuf>> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn WorkspaceProvider>| {
		Box::pin(async move { Provider.GetWorkspaceConfigurationPath().await })
	}))
}
