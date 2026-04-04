//! # ProvideSelectionRanges Effect
//!
//! Defines the `ActionEffect` for requesting selection ranges from a language feature
//! provider.

use std::sync::Arc;

use serde_json::Value;
use super::DTO::PositionDTO::PositionDTO;
use url::Url;

use super::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request selection ranges.
pub fn ProvideSelectionRanges(
	DocumentURI:Url,

	Positions:Vec<PositionDTO>,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Value>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let DocumentURIClone = DocumentURI.clone();
		let PositionsClone = Positions.clone();

		Box::pin(async move { Registry.ProvideSelectionRanges(DocumentURIClone, PositionsClone).await })
	}))
}
