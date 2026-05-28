//! # ProvideDocumentFormatting Effect
//!
//! Defines the `ActionEffect` for requesting document formatting edits from a
//! language feature provider.

use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::{DTO::TextEditDTO::TextEditDTO, LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request document formatting
/// edits.
pub fn ProvideDocumentFormatting(
	DocumentURI:Url,

	OptionsDTO:Value,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Vec<TextEditDTO>>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let DocumentURIClone = DocumentURI.clone();

		let OptionsDTOClone = OptionsDTO.clone();

		Box::pin(async move { Registry.ProvideDocumentFormattingEdits(DocumentURIClone, OptionsDTOClone).await })
	}))
}
