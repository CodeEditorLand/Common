//! # WriteFileBytes Effect
//!
//! Defines the `ActionEffect` for writing raw byte content to a file.

use std::{path::PathBuf, sync::Arc};

use super::FileSystemWriter::FileSystemWriter;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will write a vector of bytes to a
/// file at the specified path.
///
/// It uses the `FileSystemWriter` capability from the environment to perform
/// the actual file I/O.
///
/// # Parameters
/// * `Path`: The `PathBuf` of the file to write to.
/// * `Content`: The `Vec<u8>` content to be written.
/// * `Create`: If `true`, the file will be created if it does not exist.
/// * `Overwrite`: If `true`, an existing file will be overwritten.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn WriteFileBytes(
	Path:PathBuf,
	Content:Vec<u8>,
	Create:bool,
	Overwrite:bool,
) -> ActionEffect<Arc<dyn FileSystemWriter>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Writer:Arc<dyn FileSystemWriter>| {
		let PathClone = Path.clone();
		let ContentClone = Content.clone();
		Box::pin(async move { Writer.WriteFile(&PathClone, ContentClone, Create, Overwrite).await })
	}))
}
