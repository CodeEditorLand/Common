//! # CreateDirectory Effect
//!
//! Defines the `ActionEffect` for creating a new directory.

use std::{path::PathBuf, sync::Arc};

use super::FileSystemWriter::FileSystemWriter;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will create a new directory at the
/// specified path.
///
/// It uses the `FileSystemWriter` capability from the environment to perform
/// the actual file I/O.
///
/// # Parameters
/// * `Path`: The `PathBuf` of the directory to create.
/// * `Recursive`: If `true`, creates all parent directories as needed.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn CreateDirectory<TRunTime>(Path:PathBuf, Recursive:bool) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn FileSystemWriter>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let PathClone = Path.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Writer:Arc<dyn FileSystemWriter> = Environment.Require();
			Writer.CreateDirectory(&PathClone, Recursive).await
		})
	}))
}
