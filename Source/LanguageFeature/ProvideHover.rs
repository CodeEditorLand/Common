//! # ProvideHover Effect
//!
//! Defines the `ActionEffect` for requesting hover information at a specific
//! document position.

use std::sync::Arc;

use url::Url;

use super::{
	DTO::{HoverResultDTO::HoverResultDTO, PositionDTO::PositionDTO},
	LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry,
};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request hover information for a
/// symbol at a given position in a document.
/// It uses the `LanguageFeatureProviderRegistry` capability from the
/// environment to find and invoke the appropriate hover provider.
///
/// # Parameters
/// * `DocumentURI`: The `Url` of the document in which the hover was requested.
/// * `PositionDTO`: The line and column `PositionDTO` where the hover was
///   requested.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<HoverResultDTO>`, containing
/// the hover content if a provider was found and returned a result, or `None`
/// otherwise.
pub fn ProvideHover(
	DocumentURI:Url,

	PositionDTO:PositionDTO,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<HoverResultDTO>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let URIClone = DocumentURI.clone();

		Box::pin(async move { Registry.ProvideHover(URIClone, PositionDTO).await })
	}))
}
