//! # GetWorkSpaceName Effect
//!
//! Defines the `ActionEffect` for retrieving the name of the current
//! workspace.

use std::sync::Arc;

use super::WorkSpaceProvider::WorkSpaceProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will retrieve the display name of the
/// current workspace.
///
/// It uses the `WorkSpaceProvider` capability from the environment to perform
/// the operation.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<String>` containing the
/// workspace name.
pub fn GetWorkSpaceName() -> ActionEffect<Arc<dyn WorkSpaceProvider>, CommonError, Option<String>> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn WorkSpaceProvider>| {
		Box::pin(async move { Provider.GetWorkSpaceName().await })
	}))
}
