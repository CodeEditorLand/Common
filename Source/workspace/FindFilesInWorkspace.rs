use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::WorkspaceProvider::WorkspaceProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn FindFilesInWorkspace<Runtime>(
	IncludePatternDto:Value,
	ExcludePatternDto:Option<Value>,
	MaxResults:Option<usize>,
	UseIgnoreFiles:bool,
	FollowSymlinks:bool,
) -> ActionEffect<Arc<Runtime>, CommonError, Vec<Url>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn WorkspaceProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let IncludeClone = IncludePatternDto.clone();
		let ExcludeClone = ExcludePatternDto.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn WorkspaceProvider> = Environment.Require();
			Provider
				.FindFilesInWorkspace(IncludeClone, ExcludeClone, MaxResults, UseIgnoreFiles, FollowSymlinks)
				.await
		})
	}))
}
