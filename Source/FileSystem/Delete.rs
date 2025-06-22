//! # Delete Effect
//!
//! Defines the `ActionEffect` for deleting a file or directory.

use std::{path::PathBuf, sync::Arc};

use super::FileSystemWriter::FileSystemWriter;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will delete a file or directory at
/// the specified path.
///
/// It uses the `FileSystemWriter` capability from the environment to perform
/// the actual file I/O.
///
/// # Parameters
/// * `Path`: The `PathBuf` of the file or directory to delete.
/// * `Recursive`: If `true`, deletes a directory and all its contents
///   recursively. This has no effect if the path is a file.
/// * `UseTrash`: If `true`, moves the item to the system's trash or recycling
///   bin instead of permanently deleting it.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn Delete(Path:PathBuf, Recursive:bool, UseTrash:bool) -> ActionEffect<Arc<dyn FileSystemWriter>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Writer:Arc<dyn FileSystemWriter>| {
		let PathClone = Path.clone();

		Box::pin(async move { Writer.Delete(&PathClone, Recursive, UseTrash).await })
	}))
}
