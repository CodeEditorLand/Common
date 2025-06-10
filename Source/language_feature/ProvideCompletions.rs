use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::{
	LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry,
	dto::{CompletionContextDto, PositionDto, SuggestResultDto},
};
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn ProvideCompletions<Runtime>(
	DocumentUri:Url,
	PositionDto:PositionDto,
	ContextDto:CompletionContextDto,
	CancellationTokenValue:Option<Value>,
) -> ActionEffect<Arc<Runtime>, CommonError, Option<SuggestResultDto>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn LanguageFeatureProviderRegistry>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let UriClone = DocumentUri.clone();
		let ContextClone = ContextDto.clone();
		let TokenClone = CancellationTokenValue.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Registry:Arc<dyn LanguageFeatureProviderRegistry> = Environment.Require();
			Registry
				.ProvideCompletions(UriClone, PositionDto, ContextClone, TokenClone)
				.await
		})
	}))
}
