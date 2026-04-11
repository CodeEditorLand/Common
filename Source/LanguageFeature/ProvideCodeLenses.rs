//! # ProvideCodeLenses Effect
//!
//! Defines the `ActionEffect` for requesting code lenses from a language
//! feature provider.

use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request code lenses.
pub fn ProvideCodeLenses(
	DocumentURI:Url,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Value>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let DocumentURIClone = DocumentURI.clone();

		Box::pin(async move { Registry.ProvideCodeLenses(DocumentURIClone).await })
	}))
}
