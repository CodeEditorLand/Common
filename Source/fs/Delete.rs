use std::{path::PathBuf, sync::Arc};

/// @module Delete
/// @description Defines the ActionEffect for deleting a file or directory.
use super::FsWriter::FsWriter;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will delete a file or directory at
/// the specified path.
///
/// It uses the `FsWriter` capability from the environment to perform the actual
/// file I/O.
///
/// @param Path - The `PathBuf` of the file or directory to delete.
/// @param Recursive - If `true`, deletes a directory and all its contents
/// recursively. @param UseTrash - If `true`, moves the item to the system's
/// trash or recycling bin   instead of permanently deleting it.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn Delete<Runtime>(Path:PathBuf, Recursive:bool, UseTrash:bool) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn FsWriter>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let PathClone = Path.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Writer:Arc<dyn FsWriter> = Environment.Require();
			Writer.Delete(&PathClone, Recursive, UseTrash).await
		})
	}))
}
