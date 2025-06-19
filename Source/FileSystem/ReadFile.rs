//! # ReadFile Effect
//!
//! Defines the `ActionEffect` for reading the entire content of a file.

use std::{path::PathBuf, sync::Arc};

use super::FileSystemReader::FileSystemReader;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

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
pub fn ReadFile<TRunTime>(Path:PathBuf) -> ActionEffect<Arc<TRunTime>, CommonError, Vec<u8>>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn FileSystemReader>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let PathClone = Path.clone();
		Box::pin(async move {
			let Reader:Arc<dyn FileSystemReader> = RunTime.Require();
			Reader.ReadFile(&PathClone).await
		})
	}))
}
