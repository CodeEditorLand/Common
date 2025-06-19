//! # CreateFile Effect
//!
//! Defines the `ActionEffect` for creating a new, empty file.

use std::{path::PathBuf, sync::Arc};

use super::FileSystemWriter::FileSystemWriter;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will create a new, empty file at the
/// specified path.
///
/// It uses the `FileSystemWriter` capability from the environment to perform
/// the actual file I/O. This operation will typically fail if the file already
/// exists, as it is not intended to overwrite.
///
/// # Parameters
/// * `Path`: The `PathBuf` of the file to create.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn CreateFile<TRunTime>(Path:PathBuf) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn FileSystemWriter>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let PathClone = Path.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Writer:Arc<dyn FileSystemWriter> = Environment.Require();
			Writer.CreateFile(&PathClone).await
		})
	}))
}
