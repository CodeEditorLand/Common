//! # ProvideInlayHints Effect
//!
//! Defines the `ActionEffect` for requesting inlay hints from a language
//! feature provider.

use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request inlay hints.
pub fn ProvideInlayHints(
	DocumentURI:Url,

	RangeDTO:Value,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Value>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let DocumentURIClone = DocumentURI.clone();

		let RangeDTOClone = RangeDTO.clone();

		Box::pin(async move { Registry.ProvideInlayHints(DocumentURIClone, RangeDTOClone).await })
	}))
}
