//! # GetWorkSpaceConfigurationPath Effect
//!
//! Defines the `ActionEffect` for retrieving the path to the workspace's
//! configuration file.

use std::{path::PathBuf, sync::Arc};

use super::WorkSpaceProvider::WorkSpaceProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will retrieve the file path of the
/// current workspace's configuration file (e.g., the `.code-workspace` file).
///
/// It uses the `WorkSpaceProvider` capability from the environment.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<PathBuf>`, containing the
/// path if a workspace configuration file is open, or `None` otherwise.
pub fn GetWorkSpaceConfigurationPath<TRunTime>() -> ActionEffect<Arc<TRunTime>, CommonError, Option<PathBuf>>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn WorkSpaceProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Provider:Arc<dyn WorkSpaceProvider> = Environment.Require();
			Provider.GetWorkSpaceConfigurationPath().await
		})
	}))
}
