//! # Rename Effect
//!
//! Defines the `ActionEffect` for renaming or moving a file or directory.

use std::{path::PathBuf, sync::Arc};

use super::FileSystemWriter::FileSystemWriter;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will rename (or move) a file or
/// directory from a source path to a target path.
///
/// It uses the `FileSystemWriter` capability from the environment to perform
/// the actual file I/O.
///
/// # Parameters
/// * `Source`: The `PathBuf` of the file or directory to rename.
/// * `Target`: The `PathBuf` of the new name or location.
/// * `Overwrite`: If `true`, an existing file or directory at the target path
///   will be overwritten.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn Rename(
	Source:PathBuf,

	Target:PathBuf,

	Overwrite:bool,
) -> ActionEffect<Arc<dyn FileSystemWriter>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Writer:Arc<dyn FileSystemWriter>| {
		let SourceClone = Source.clone();

		let TargetClone = Target.clone();

		Box::pin(async move { Writer.Rename(&SourceClone, &TargetClone, Overwrite).await })
	}))
}
