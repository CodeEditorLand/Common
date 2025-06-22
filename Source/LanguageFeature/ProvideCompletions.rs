//! # ProvideCompletions Effect
//!
//! Defines the `ActionEffect` for requesting completion items at a specific
//! document position.

use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::{
	DTO::{CompletionContextDTO::CompletionContextDTO, CompletionListDTO::CompletionListDTO, PositionDTO::PositionDTO},
	LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry,
};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request code completion items
/// for a given position in a document.
pub fn ProvideCompletions(
	DocumentURI:Url,
	PositionDTO:PositionDTO,
	ContextDTO:CompletionContextDTO,
	CancellationTokenValue:Option<Value>,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<CompletionListDTO>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let URIClone = DocumentURI.clone();
		// PositionDTO is Copy
		let PositionClone = PositionDTO;
		let ContextClone = ContextDTO.clone();
		let TokenClone = CancellationTokenValue.clone();
		Box::pin(async move {
			Registry
				.ProvideCompletions(URIClone, PositionClone, ContextClone, TokenClone)
				.await
		})
	}))
}
