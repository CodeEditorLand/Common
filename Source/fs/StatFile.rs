use std::{path::PathBuf, sync::Arc};

/// @module StatFile
/// @description Defines the ActionEffect for retrieving metadata about a file
/// or directory.
use super::{FsReader::FsReader, dto::FileSystemStatDto};
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will retrieve metadata (such as file
/// type, size, and modification times) for a given path.
///
/// It uses the `FsReader` capability from the environment to perform the actual
/// `stat` system call.
///
/// @param Path - The `PathBuf` of the file or directory to stat.
///
/// @returns An `ActionEffect` that resolves with a `FileSystemStatDto`.
pub fn StatFile<Runtime>(Path:PathBuf) -> ActionEffect<Arc<Runtime>, CommonError, FileSystemStatDto>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn FsReader>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let PathClone = Path.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Reader:Arc<dyn FsReader> = Environment.Require();
			Reader.StatFile(&PathClone).await
		})
	}))
}
