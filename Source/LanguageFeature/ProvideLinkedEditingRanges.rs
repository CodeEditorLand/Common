//! # ProvideLinkedEditingRanges Effect
//!
//! Defines the `ActionEffect` for requesting linked editing ranges from a
//! language feature provider.

use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::{DTO::PositionDTO::PositionDTO, LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request linked editing ranges.
pub fn ProvideLinkedEditingRanges(
	DocumentURI:Url,

	PositionDTO:PositionDTO,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Value>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let DocumentURIClone = DocumentURI.clone();

		Box::pin(async move { Registry.ProvideLinkedEditingRanges(DocumentURIClone, PositionDTO).await })
	}))
}
