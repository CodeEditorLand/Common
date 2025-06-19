//! # ReadDirectory Effect
//!
//! Defines the `ActionEffect` for reading the contents of a directory.

use std::{path::PathBuf, sync::Arc};

use super::{DTO::FileTypeDTO::FileTypeDTO, FileSystemReader::FileSystemReader};
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will read the entries of a directory
/// at the specified path.
///
/// It uses the `FileSystemReader` capability from the environment to perform
/// the actual file I/O.
///
/// # Parameters
/// * `Path`: The `PathBuf` of the directory to read.
///
/// # Returns
/// An `ActionEffect` that resolves with a `Vec` of tuples, where each tuple
/// contains the entry's name (`String`) and its `FileTypeDTO`.
pub fn ReadDirectory<TRunTime>(Path:PathBuf) -> ActionEffect<Arc<TRunTime>, CommonError, Vec<(String, FileTypeDTO)>>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn FileSystemReader>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let PathClone = Path.clone();
		Box::pin(async move {
			let Reader:Arc<dyn FileSystemReader> = RunTime.Require();
			Reader.ReadDirectory(&PathClone).await
		})
	}))
}
