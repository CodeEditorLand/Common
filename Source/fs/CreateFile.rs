use std::{path::PathBuf, sync::Arc};

/// @module CreateFile
/// @description Defines the ActionEffect for creating a new, empty file.
use super::FsWriter::FsWriter;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will create a new, empty file at the
/// specified path.
///
/// It uses the `FsWriter` capability from the environment to perform the actual
/// file I/O. This operation will typically fail if the file already exists.
///
/// @param Path - The `PathBuf` of the file to create.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn CreateFile<Runtime>(Path:PathBuf) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn FsWriter>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let PathClone = Path.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Writer:Arc<dyn FsWriter> = Environment.Require();
			Writer.CreateFile(&PathClone).await
		})
	}))
}
