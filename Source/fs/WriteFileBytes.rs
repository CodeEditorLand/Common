use std::{path::PathBuf, sync::Arc};

/// @module WriteFileBytes
/// @description Defines the ActionEffect for writing byte content to a file.
use super::FsWriter::FsWriter;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will write a vector of bytes to a
/// file at the specified path.
///
/// It uses the `FsWriter` capability from the environment to perform the actual
/// file I/O.
///
/// @param Path - The `PathBuf` of the file to write to.
/// @param Content - The `Vec<u8>` content to be written.
/// @param Create - If `true`, the file will be created if it does not exist.
/// @param Overwrite - If `true`, an existing file will be overwritten.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn WriteFileBytes<Runtime>(
	Path:PathBuf,
	Content:Vec<u8>,
	Create:bool,
	Overwrite:bool,
) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn FsWriter>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let PathClone = Path.clone();
		let ContentClone = Content.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Writer:Arc<dyn FsWriter> = Environment.Require();
			Writer.WriteFile(&PathClone, ContentClone, Create, Overwrite).await
		})
	}))
}
