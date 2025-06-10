use std::{path::PathBuf, sync::Arc};

/// @module Copy
/// @description Defines the ActionEffect for copying a file or directory.
use super::FsWriter::FsWriter;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will copy a file or directory from a
/// source path to a target path.
///
/// It uses the `FsWriter` capability from the environment to perform the actual
/// file I/O.
///
/// @param Source - The `PathBuf` of the file or directory to copy.
/// @param Target - The `PathBuf` of the destination.
/// @param Overwrite - If `true`, an existing file or directory at the target
/// path   will be overwritten.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn Copy<Runtime>(Source:PathBuf, Target:PathBuf, Overwrite:bool) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn FsWriter>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let SourceClone = Source.clone();
		let TargetClone = Target.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Writer:Arc<dyn FsWriter> = Environment.Require();
			Writer.Copy(&SourceClone, &TargetClone, Overwrite).await
		})
	}))
}
