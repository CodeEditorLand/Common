//! # OpenFile Effect
//!
//! Defines the `ActionEffect` for requesting that a file be opened in an
//! editor.

use std::{path::PathBuf, sync::Arc};

use super::WorkSpaceProvider::WorkSpaceProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request that the host
/// application open the specified file path in an editor.
///
/// It uses the `WorkSpaceProvider` capability from the environment. The actual
/// implementation will likely involve creating a new document model (if one
/// doesn't exist) and sending an event to the UI to reveal an editor for it.
///
/// # Parameters
/// * `Path`: The `PathBuf` of the file to open.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn OpenFile(Path:PathBuf) -> ActionEffect<Arc<dyn WorkSpaceProvider>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn WorkSpaceProvider>| {
		let PathClone = Path.clone();

		Box::pin(async move { Provider.OpenFile(PathClone).await })
	}))
}
