//! # IsWorkSpaceTrusted Effect
//!
//! Defines the `ActionEffect` for checking if the current workspace is trusted.

use std::sync::Arc;

use super::WorkSpaceProvider::WorkSpaceProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will check if the current workspace
/// is considered trusted by the user.
///
/// WorkSpace Trust is a security feature that restricts certain operations
/// (like automatic task execution) in untrusted folders.
///
/// It uses the `WorkSpaceProvider` capability from the environment.
///
/// # Returns
/// An `ActionEffect` that resolves with a `bool` indicating the trust state.
pub fn IsWorkSpaceTrusted() -> ActionEffect<Arc<dyn WorkSpaceProvider>, CommonError, bool> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn WorkSpaceProvider>| {
		Box::pin(async move { Provider.IsWorkSpaceTrusted().await })
	}))
}
