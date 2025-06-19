//! # RegisterProvider Effect
//!
//! Defines the `ActionEffect` for registering a new language feature provider.

use std::sync::Arc;

use serde_json::Value;

use super::{DTO::ProviderType::ProviderType, LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry};
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will register a new language feature
/// provider with the host's central registry.
///
/// It uses the `LanguageFeatureProviderRegistry` capability from the
/// environment.
///
/// # Parameters
/// * `ProviderType`: The enum variant identifying the feature type.
/// * `SelectorDTO`: The document selector that determines when this provider is
///   active.
/// * `SidecarIdentifier`: The ID of the sidecar hosting the provider logic.
/// * `ExtensionIdentifierDTO`: The ID of the extension contributing the
///   provider.
/// * `OptionsDTO`: Optional, feature-specific options.
///
/// # Returns
/// An `ActionEffect` that resolves with a unique `u32` handle for the
/// registration.
pub fn RegisterProvider<TRunTime>(
	ProviderType:ProviderType,
	SelectorDTO:Value,
	SidecarIdentifier:String,
	ExtensionIdentifierDTO:Value,
	OptionsDTO:Option<Value /* ProviderOptionsDTO */>,
) -> ActionEffect<Arc<TRunTime>, CommonError, u32>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn LanguageFeatureProviderRegistry>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let SelectorClone = SelectorDTO.clone();
		let SidecarIdentifierClone = SidecarIdentifier.clone();
		let ExtensionIdentifierClone = ExtensionIdentifierDTO.clone();
		let OptionsClone = OptionsDTO.clone();
		Box::pin(async move {
			let Registry:Arc<dyn LanguageFeatureProviderRegistry> = RunTime.Require();
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
