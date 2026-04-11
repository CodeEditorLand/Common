//! # ProvideDocumentSymbols Effect
//!
//! Defines the `ActionEffect` for requesting document symbols from a language
//! feature provider.

use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request document symbols.
pub fn ProvideDocumentSymbols(
	DocumentURI:Url,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Value>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let DocumentURIClone = DocumentURI.clone();

		Box::pin(async move { Registry.ProvideDocumentSymbols(DocumentURIClone).await })
	}))
}
