use std::sync::Arc;

use super::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn UnregisterProvider<Runtime>(Handle:u32) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn LanguageFeatureProviderRegistry>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Registry:Arc<dyn LanguageFeatureProviderRegistry> = Environment.Require();
			Registry.UnregisterProvider(Handle).await
		})
	}))
}
