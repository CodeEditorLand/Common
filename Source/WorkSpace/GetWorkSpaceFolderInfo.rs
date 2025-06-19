//! # GetWorkSpaceFolderInfo Effect
//!
//! Defines the `ActionEffect` for retrieving information about the specific
//! workspace folder that contains a given URI.

use std::sync::Arc;

use url::Url;

use super::WorkSpaceProvider::WorkSpaceProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will find the workspace folder that
/// contains the given URI.
///
/// It uses the `WorkSpaceProvider` capability from the environment. This is
/// useful for determining which folder-level settings apply to a specific
/// file.
///
/// # Parameters
/// * `URIToMatch`: The `Url` of the resource (e.g., a file) for which to find
///   the containing workspace folder.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option` containing a tuple of the
/// folder's `Url`, name, and index, or `None` if the URI is not within any
/// open workspace folder.
pub fn GetWorkSpaceFolderInfo<TRunTime>(
	URIToMatch:Url,
) -> ActionEffect<Arc<TRunTime>, CommonError, Option<(Url, String, usize)>>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn WorkSpaceProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let URIClone = URIToMatch.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Provider:Arc<dyn WorkSpaceProvider> = Environment.Require();
			Provider.GetWorkSpaceFolderInfo(URIClone).await
		})
	}))
}
