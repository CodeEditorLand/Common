//! # ProvideCompletions Effect
//!
//! Defines the `ActionEffect` for requesting completion items at a specific
//! document position.

use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::{
	DTO::{PositionDTO::PositionDTO /* , CompletionContextDTO, SuggestResultDTO */}, // DTOs to be added
	LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry,
};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request code completion items
/// for a given position in a document.
///
/// It uses the `LanguageFeatureProviderRegistry` capability from the
/// environment to find and invoke the appropriate completion provider.
///
/// # Parameters
/// * `DocumentURI`: The `Url` of the document.
/// * `PositionDTO`: The line and column `PositionDTO` where completions are
///   requested.
/// * `ContextDTO`: A `Value` representing the `CompletionContextDTO`, providing
///   information about how the completion was triggered.
/// * `CancellationTokenValue`: A `Value` representing a cancellation token.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option` containing the completion
/// results (a `SuggestResultDTO` serialized as a `Value`), or `None`.
pub fn ProvideCompletions(
	DocumentURI:Url,
	PositionDTO:PositionDTO,
	ContextDTO:Value, // To be replaced with concrete DTO
	CancellationTokenValue:Option<Value>,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Value /* SuggestResultDTO */>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let URIClone = DocumentURI.clone();
		let ContextClone = ContextDTO.clone();
		let TokenClone = CancellationTokenValue.clone();
		Box::pin(async move {
			Registry
				.ProvideCompletions(URIClone, PositionDTO, ContextClone, TokenClone)
				.await
		})
	}))
}
