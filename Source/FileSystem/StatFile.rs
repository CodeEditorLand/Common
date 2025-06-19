//! # StatFile Effect
//!
//! Defines the `ActionEffect` for retrieving metadata about a file or
//! directory.

use std::{path::PathBuf, sync::Arc};

use super::{DTO::FileSystemStatDTO::FileSystemStatDTO, FileSystemReader::FileSystemReader};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will retrieve metadata (such as file
/// type, size, and modification times) for a given path.
///
/// It uses the `FileSystemReader` capability from the environment to perform
/// the underlying `stat` system call asynchronously.
///
/// # Parameters
/// * `Path`: The `PathBuf` of the file or directory to stat.
///
/// # Returns
/// An `ActionEffect` that resolves with a `FileSystemStatDTO`.
pub fn StatFile(Path:PathBuf) -> ActionEffect<Arc<dyn FileSystemReader>, CommonError, FileSystemStatDTO> {
	ActionEffect::New(Arc::new(move |Reader:Arc<dyn FileSystemReader>| {
		let PathClone = Path.clone();
		Box::pin(async move { Reader.StatFile(&PathClone).await })
	}))
}
