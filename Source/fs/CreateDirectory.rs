use std::{path::PathBuf, sync::Arc};

/// @module CreateDirectory
/// @description Defines the ActionEffect for creating a directory.
use super::FsWriter::FsWriter;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will create a new directory at the
/// specified path.
///
/// It uses the `FsWriter` capability from the environment to perform the actual
/// file I/O.
///
/// @param Path - The `PathBuf` of the directory to create.
/// @param Recursive - If `true`, creates all parent directories as needed.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn CreateDirectory<Runtime>(Path:PathBuf, Recursive:bool) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn FsWriter>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let PathClone = Path.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Writer:Arc<dyn FsWriter> = Environment.Require();
			Writer.CreateDirectory(&PathClone, Recursive).await
		})
	}))
}
