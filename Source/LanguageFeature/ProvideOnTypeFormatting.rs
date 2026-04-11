//! # ProvideOnTypeFormatting Effect
//!
//! Defines the `ActionEffect` for requesting on-type formatting edits from a
//! language feature provider.

use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::{
	DTO::{PositionDTO::PositionDTO, TextEditDTO::TextEditDTO},
	LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry,
};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request on-type formatting
/// edits.
pub fn ProvideOnTypeFormatting(
	DocumentURI:Url,

	PositionDTO:PositionDTO,

	Character:String,

	OptionsDTO:Value,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Vec<TextEditDTO>>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let DocumentURIClone = DocumentURI.clone();
		let CharacterClone = Character.clone();
		let OptionsDTOClone = OptionsDTO.clone();

		Box::pin(async move {
			Registry
				.ProvideOnTypeFormattingEdits(DocumentURIClone, PositionDTO, CharacterClone, OptionsDTOClone)
				.await
		})
	}))
}
