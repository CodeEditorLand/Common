//! # ProvideReferences Effect
//!
//! Defines the `ActionEffect` for requesting reference locations from a language feature
//! provider.

use std::sync::Arc;

use serde_json::Value;
use super::DTO::LocationDTO::LocationDTO;
use super::DTO::PositionDTO::PositionDTO;
use url::Url;

use super::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request reference locations.
pub fn ProvideReferences(
	DocumentURI:Url,

	PositionDTO:PositionDTO,

	ContextDTO:Value,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Vec<LocationDTO>>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let DocumentURIClone = DocumentURI.clone();
		let ContextDTOClone = ContextDTO.clone();

		Box::pin(async move { Registry.ProvideReferences(DocumentURIClone, PositionDTO, ContextDTOClone).await })
	}))
}
