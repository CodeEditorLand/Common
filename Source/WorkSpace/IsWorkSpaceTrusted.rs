//! # IsWorkSpaceTrusted Effect
//!
//! Defines the `ActionEffect` for checking if the current workspace is trusted.

use std::sync::Arc;

use super::WorkSpaceProvider::WorkSpaceProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will check if the current workspace
/// is considered trusted by the user.
///
/// WorkSpace Trust is a security feature that restricts certain operations
/// (like automatic task execution) in untrusted folders.
///
/// It uses the `WorkSpaceProvider` capability from the environment.
///
/// # Returns
/// An `ActionEffect` that resolves with a `bool` indicating the trust state.
pub fn IsWorkSpaceTrusted<TRunTime>() -> ActionEffect<Arc<TRunTime>, CommonError, bool>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn WorkSpaceProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		Box::pin(async move {
			let Provider:Arc<dyn WorkSpaceProvider> = RunTime.Require();
			Provider.IsWorkSpaceTrusted().await
		})
	}))
}
