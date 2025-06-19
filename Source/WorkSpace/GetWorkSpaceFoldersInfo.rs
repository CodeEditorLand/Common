//! # GetWorkSpaceFoldersInfo Effect
//!
//! Defines the `ActionEffect` for retrieving information about all open
//! workspace folders.

use std::sync::Arc;

use url::Url;

use super::WorkSpaceProvider::WorkSpaceProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will retrieve information about all
/// currently open workspace folders.
///
/// It uses the `WorkSpaceProvider` capability from the environment to perform
/// the operation.
///
/// # Returns
/// An `ActionEffect` that resolves with a `Vec` of tuples, where each tuple
/// contains the folder's `Url`, its name as a `String`, and its zero-based
/// index.
pub fn GetWorkSpaceFoldersInfo<TRunTime>() -> ActionEffect<Arc<TRunTime>, CommonError, Vec<(Url, String, usize)>>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn WorkSpaceProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		Box::pin(async move {
			let Provider:Arc<dyn WorkSpaceProvider> = RunTime.Require();
			Provider.GetWorkSpaceFoldersInfo().await
		})
	}))
}
