//! # ReadDirectory Effect
//!
//! Defines the `ActionEffect` for reading the contents of a directory.

use std::{path::PathBuf, sync::Arc};

use super::{DTO::FileTypeDTO::FileTypeDTO, FileSystemReader::FileSystemReader};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will read the entries of a directory
/// at the specified path.
/// It uses the `FileSystemReader` capability from the environment to perform
/// the actual file I/O.
///
/// # Parameters
/// * `Path`: The `PathBuf` of the directory to read.
///
/// # Returns
/// An `ActionEffect` that resolves with a `Vec` of tuples, where each tuple
/// contains the entry's name (`String`) and its `FileTypeDTO`.
pub fn ReadDirectory(Path:PathBuf) -> ActionEffect<Arc<dyn FileSystemReader>, CommonError, Vec<(String, FileTypeDTO)>> {
	ActionEffect::New(Arc::new(move |Reader:Arc<dyn FileSystemReader>| {
		let PathClone = Path.clone();

		Box::pin(async move { Reader.ReadDirectory(&PathClone).await })
	}))
}
