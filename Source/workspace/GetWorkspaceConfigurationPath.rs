use std::{path::PathBuf, sync::Arc};

use super::WorkspaceProvider::WorkspaceProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn GetWorkspaceConfigurationPath<Runtime>() -> ActionEffect<Arc<Runtime>, CommonError, Option<PathBuf>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn WorkspaceProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn WorkspaceProvider> = Environment.Require();
			Provider.GetWorkspaceConfigurationPath().await
		})
	}))
}
