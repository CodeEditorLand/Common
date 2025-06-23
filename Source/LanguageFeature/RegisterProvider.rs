//! # RegisterProvider Effect
//!
//! Defines the `ActionEffect` for registering a new language feature provider.

use std::sync::Arc;

use serde_json::Value;

use super::{DTO::ProviderType::ProviderType, LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

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
/// * `SideCarIdentifier`: The ID of the sidecar hosting the provider logic.
/// * `ExtensionIdentifierDTO`: The ID of the extension contributing the
///   provider.
/// * `OptionsDTO`: Optional, feature-specific options.
///
/// # Returns
/// An `ActionEffect` that resolves with a unique `u32` handle for the
/// registration.
pub fn RegisterProvider(
	ProviderType:ProviderType,

	SelectorDTO:Value,

	SideCarIdentifier:String,

	ExtensionIdentifierDTO:Value,

	OptionsDTO:Option<Value /* ProviderOptionsDTO */>,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, u32> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let SelectorClone = SelectorDTO.clone();

		let SideCarIdentifierClone = SideCarIdentifier.clone();

		let ExtensionIdentifierClone = ExtensionIdentifierDTO.clone();

		let OptionsClone = OptionsDTO.clone();

		Box::pin(async move {
			Registry
				.RegisterProvider(
					SideCarIdentifierClone,
					ProviderType,
					SelectorClone,
					ExtensionIdentifierClone,
					OptionsClone,
				)
				.await
		})
	}))
}
