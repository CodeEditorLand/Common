//! # OpenFile Effect
//!
//! Defines the `ActionEffect` for requesting that a file be opened in an
//! editor.

use std::{path::PathBuf, sync::Arc};

use super::WorkSpaceProvider::WorkSpaceProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will request that the host
/// application open the specified file path in an editor.
///
/// It uses the `WorkSpaceProvider` capability from the environment. The actual
/// implementation will likely involve creating a new document model (if one
/// doesn't exist) and sending an event to the UI to reveal an editor for it.
///
/// # Parameters
/// * `Path`: The `PathBuf` of the file to open.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn OpenFile<TRunTime>(Path:PathBuf) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn WorkSpaceProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let PathClone = Path.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Provider:Arc<dyn WorkSpaceProvider> = Environment.Require();
			Provider.OpenFile(PathClone).await
		})
	}))
}
