use std::{path::PathBuf, sync::Arc};

/// @module Rename
/// @description Defines the ActionEffect for renaming (or moving) a file or
/// directory.
use super::FsWriter::FsWriter;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will rename (or move) a file or
/// directory from a source path to a target path.
///
/// It uses the `FsWriter` capability from the environment to perform the actual
/// file I/O.
///
/// @param Source - The `PathBuf` of the file or directory to rename.
/// @param Target - The `PathBuf` of the new name or location.
/// @param Overwrite - If `true`, an existing file or directory at the target
/// path   will be overwritten.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn Rename<Runtime>(Source:PathBuf, Target:PathBuf, Overwrite:bool) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn FsWriter>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let SourceClone = Source.clone();
		let TargetClone = Target.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Writer:Arc<dyn FsWriter> = Environment.Require();
			Writer.Rename(&SourceClone, &TargetClone, Overwrite).await
		})
	}))
}
