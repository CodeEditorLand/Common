//! # ProvideDefinition Effect
//!
//! Defines the `ActionEffect` for requesting definition locations from a
//! language feature provider.

use std::sync::Arc;

use url::Url;

use super::{
	DTO::{LocationDTO::LocationDTO, PositionDTO::PositionDTO},
	LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry,
};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request definition locations.
pub fn ProvideDefinition(
	DocumentURI:Url,

	PositionDTO:PositionDTO,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Vec<LocationDTO>>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let DocumentURIClone = DocumentURI.clone();

		Box::pin(async move { Registry.ProvideDefinition(DocumentURIClone, PositionDTO).await })
	}))
}
