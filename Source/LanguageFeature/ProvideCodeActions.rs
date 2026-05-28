//! # ProvideCodeActions Effect
//!
//! Defines the `ActionEffect` for requesting code actions from a language
//! feature provider.

use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request code actions.
pub fn ProvideCodeActions(
	DocumentURI:Url,

	RangeOrSelectionDTO:Value,

	ContextDTO:Value,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Value>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let DocumentURIClone = DocumentURI.clone();

		let RangeOrSelectionDTOClone = RangeOrSelectionDTO.clone();

		let ContextDTOClone = ContextDTO.clone();

		Box::pin(async move {
			Registry
				.ProvideCodeActions(DocumentURIClone, RangeOrSelectionDTOClone, ContextDTOClone)
				.await
		})
	}))
}
