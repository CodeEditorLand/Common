//! # CreateDirectory Effect
//!
//! Defines the `ActionEffect` for creating a new directory.

use std::{path::PathBuf, sync::Arc};

use super::FileSystemWriter::FileSystemWriter;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will create a new directory at the
/// specified path.
/// It uses the `FileSystemWriter` capability from the environment to perform
/// the actual file I/O.
///
/// # Parameters
/// * `Path`: The `PathBuf` of the directory to create.
/// * `Recursive`: If `true`, creates all parent directories as needed.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn CreateDirectory(Path:PathBuf, Recursive:bool) -> ActionEffect<Arc<dyn FileSystemWriter>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Writer:Arc<dyn FileSystemWriter>| {
		let PathClone = Path.clone();

		Box::pin(async move { Writer.CreateDirectory(&PathClone, Recursive).await })
	}))
}
