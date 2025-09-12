//! # ReadFile Effect
//!
//! Defines the `ActionEffect` for reading the entire content of a file.

use std::{path::PathBuf, sync::Arc};

use super::FileSystemReader::FileSystemReader;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will read the entire contents of a
/// file at the specified path into a byte vector.
///
/// It uses the `FileSystemReader` capability from the environment to perform
/// the actual file I/O.
///
/// # Parameters
/// * `Path`: The `PathBuf` of the file to read.
///
/// # Returns
/// An `ActionEffect` that resolves with a `Vec<u8>` containing the file's
/// content.
pub fn ReadFile(Path:PathBuf) -> ActionEffect<Arc<dyn FileSystemReader>, CommonError, Vec<u8>> {
	ActionEffect::New(Arc::new(move |Reader:Arc<dyn FileSystemReader>| {
		let PathClone = Path.clone();

		Box::pin(async move { Reader.ReadFile(&PathClone).await })
	}))
}
