use std::sync::Arc;

use url::Url;

use super::WorkspaceProvider::WorkspaceProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn GetWorkspaceFoldersInfo<Runtime>() -> ActionEffect<Arc<Runtime>, CommonError, Vec<(Url, String, usize)>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn WorkspaceProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn WorkspaceProvider> = Environment.Require();
			Provider.GetWorkspaceFoldersInfo().await
		})
	}))
}
