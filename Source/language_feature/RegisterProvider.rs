use std::sync::Arc;

use serde_json::Value;

use super::{
	LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry,
	dto::{ProviderOptionsDto, ProviderType},
};
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn RegisterProvider<Runtime>(
	ProviderType:ProviderType,
	SelectorDto:Value,
	SidecarIdentifier:String,
	ExtensionIdentifierDto:Value,
	OptionsDto:Option<ProviderOptionsDto>,
) -> ActionEffect<Arc<Runtime>, CommonError, u32>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn LanguageFeatureProviderRegistry>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let SelectorClone = SelectorDto.clone();
		let SidecarIdentifierClone = SidecarIdentifier.clone();
		let ExtensionIdentifierClone = ExtensionIdentifierDto.clone();
		let OptionsClone = OptionsDto.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Registry:Arc<dyn LanguageFeatureProviderRegistry> = Environment.Require();
			Registry
				.RegisterProvider(
					SidecarIdentifierClone,
					ProviderType,
					SelectorClone,
					ExtensionIdentifierClone,
					OptionsClone,
				)
				.await
		})
	}))
}
