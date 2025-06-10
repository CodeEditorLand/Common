use std::{path::PathBuf, sync::Arc};

/// @module ReadDirectory
/// @description Defines the ActionEffect for reading the contents of a
/// directory.
use super::{FsReader::FsReader, dto::FileTypeDto};
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will read the entries of a directory
/// at the specified path.
///
/// It uses the `FsReader` capability from the environment to perform the actual
/// file I/O.
///
/// @param Path - The `PathBuf` of the directory to read.
///
/// @returns An `ActionEffect` that resolves with a `Vec` of tuples, where each
///   tuple contains the entry's name and its `FileTypeDto`.
pub fn ReadDirectory<Runtime>(Path:PathBuf) -> ActionEffect<Arc<Runtime>, CommonError, Vec<(String, FileTypeDto)>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn FsReader>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let PathClone = Path.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Reader:Arc<dyn FsReader> = Environment.Require();
			Reader.ReadDirectory(&PathClone).await
		})
	}))
}
