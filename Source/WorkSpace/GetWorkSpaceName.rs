//! # GetWorkSpaceName Effect
//!
//! Defines the `ActionEffect` for retrieving the name of the current
//! workspace.

use std::sync::Arc;

use super::WorkSpaceProvider::WorkSpaceProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will retrieve the display name of the
/// current workspace.
///
/// It uses the `WorkSpaceProvider` capability from the environment to perform
/// the operation.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<String>` containing the
/// workspace name.
pub fn GetWorkSpaceName<TRunTime>() -> ActionEffect<Arc<TRunTime>, CommonError, Option<String>>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn WorkSpaceProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		Box::pin(async move {
			let Provider:Arc<dyn WorkSpaceProvider> = RunTime.Require();
			Provider.GetWorkSpaceName().await
		})
	}))
}
