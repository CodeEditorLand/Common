//! # FindFilesInWorkSpace Effect
//!
//! Defines the `ActionEffect` for finding files within the workspace that
//! match given glob patterns.

use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::WorkSpaceProvider::WorkSpaceProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will find files within the workspace
/// based on include and exclude glob patterns.
///
/// It uses the `WorkSpaceProvider` capability from the environment to perform
/// the search.
///
/// # Parameters
/// * `IncludePatternDTO`: A `serde_json::Value` representing the glob pattern
///   to include.
/// * `ExcludePatternDTO`: An optional `serde_json::Value` for files/folders to
///   exclude.
/// * `MaxResults`: An optional limit on the number of results to return.
/// * `UseIgnoreFiles`: Whether to respect `.gitignore`-style ignore files.
/// * `FollowSymlinks`: Whether to follow symbolic links during the search.
///
/// # Returns
/// An `ActionEffect` that resolves with a `Vec<Url>` of the matching file URIs.
pub fn FindFilesInWorkSpace<TRunTime>(
	IncludePatternDTO:Value,
	ExcludePatternDTO:Option<Value>,
	MaxResults:Option<usize>,
	UseIgnoreFiles:bool,
	FollowSymlinks:bool,
) -> ActionEffect<Arc<TRunTime>, CommonError, Vec<Url>>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn WorkSpaceProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let IncludeClone = IncludePatternDTO.clone();
		let ExcludeClone = ExcludePatternDTO.clone();
		Box::pin(async move {
			let Provider:Arc<dyn WorkSpaceProvider> = RunTime.Require();
			Provider
				.FindFilesInWorkSpace(IncludeClone, ExcludeClone, MaxResults, UseIgnoreFiles, FollowSymlinks)
				.await
		})
	}))
}
