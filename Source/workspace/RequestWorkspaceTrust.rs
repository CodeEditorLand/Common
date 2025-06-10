use std::sync::Arc;

use serde_json::Value;

use super::WorkspaceProvider::WorkspaceProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn RequestWorkspaceTrust<Runtime>(Options:Option<Value>) -> ActionEffect<Arc<Runtime>, CommonError, bool>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn WorkspaceProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let OptionsClone = Options.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn WorkspaceProvider> = Environment.Require();
			Provider.RequestWorkspaceTrust(OptionsClone).await
		})
	}))
}
