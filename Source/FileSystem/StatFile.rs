//! # StatFile Effect
//!
//! Defines the `ActionEffect` for retrieving metadata about a file or
//! directory.

use std::{path::PathBuf, sync::Arc};

use super::{DTO::FileSystemStatDTO::FileSystemStatDTO, FileSystemReader::FileSystemReader};
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

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
pub fn StatFile<TRunTime>(Path:PathBuf) -> ActionEffect<Arc<TRunTime>, CommonError, FileSystemStatDTO>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn FileSystemReader>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let PathClone = Path.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Reader:Arc<dyn FileSystemReader> = Environment.Require();
			Reader.StatFile(&PathClone).await
		})
	}))
}
