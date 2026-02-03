//! # GetWorkspaceFoldersInfo Effect
//!
//! Defines the `ActionEffect` for retrieving information about all open
//! workspace folders.

use std::sync::Arc;

use url::Url;

use super::WorkspaceProvider::WorkspaceProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will retrieve information about all
/// currently open workspace folders.
///
/// It uses the `WorkspaceProvider` capability from the environment to perform
/// the operation.
///
/// # Returns
/// An `ActionEffect` that resolves with a `Vec` of tuples, where each tuple
/// contains the folder's `Url`, its name as a `String`, and its zero-based
/// index.
pub fn GetWorkspaceFoldersInfo() -> ActionEffect<Arc<dyn WorkspaceProvider>, CommonError, Vec<(Url, String, usize)>> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn WorkspaceProvider>| {
		Box::pin(async move { Provider.GetWorkspaceFoldersInfo().await })
	}))
}
