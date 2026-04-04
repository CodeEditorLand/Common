//! # ProvideSignatureHelp Effect
//!
//! Defines the `ActionEffect` for requesting signature help from a language feature
//! provider.

use std::sync::Arc;

use serde_json::Value;
use super::DTO::PositionDTO::PositionDTO;
use url::Url;

use super::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request signature help.
pub fn ProvideSignatureHelp(
	DocumentURI:Url,

	PositionDTO:PositionDTO,

	ContextDTO:Value,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Value>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let DocumentURIClone = DocumentURI.clone();
		let ContextDTOClone = ContextDTO.clone();

		Box::pin(async move { Registry.ProvideSignatureHelp(DocumentURIClone, PositionDTO, ContextDTOClone).await })
	}))
}
