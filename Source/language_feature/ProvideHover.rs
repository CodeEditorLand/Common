use std::sync::Arc;

use url::Url;

use super::{
	LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry,
	dto::{HoverResultDto, PositionDto},
};
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn ProvideHover<Runtime>(
	DocumentUri:Url,
	PositionDto:PositionDto,
) -> ActionEffect<Arc<Runtime>, CommonError, Option<HoverResultDto>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn LanguageFeatureProviderRegistry>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let UriClone = DocumentUri.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Registry:Arc<dyn LanguageFeatureProviderRegistry> = Environment.Require();
			Registry.ProvideHover(UriClone, PositionDto).await
		})
	}))
}
