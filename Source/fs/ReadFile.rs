use std::{path::PathBuf, sync::Arc};

/// @module ReadFile
/// @description Defines the ActionEffect for reading the content of a file.
use super::FsReader::FsReader;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will read the entire contents of a
/// file at the specified path into a byte vector.
///
/// It uses the `FsReader` capability from the environment to perform the actual
/// file I/O.
///
/// @param Path - The `PathBuf` of the file to read.
///
/// @returns An `ActionEffect` that resolves with a `Vec<u8>` containing the
/// file's content.
pub fn ReadFile<Runtime>(Path:PathBuf) -> ActionEffect<Arc<Runtime>, CommonError, Vec<u8>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn FsReader>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let PathClone = Path.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Reader:Arc<dyn FsReader> = Environment.Require();
			Reader.ReadFile(&PathClone).await
		})
	}))
}
