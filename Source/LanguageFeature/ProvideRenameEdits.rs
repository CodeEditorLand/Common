//! # ProvideRenameEdits Effect
//!
//! Defines the `ActionEffect` for requesting rename edits from a language feature
//! provider.

use std::sync::Arc;

use serde_json::Value;
use super::DTO::PositionDTO::PositionDTO;
use url::Url;

use super::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request rename edits.
pub fn ProvideRenameEdits(
	DocumentURI:Url,

	PositionDTO:PositionDTO,

	NewName:String,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Value>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let DocumentURIClone = DocumentURI.clone();
		let NewNameClone = NewName.clone();

		Box::pin(async move { Registry.ProvideRenameEdits(DocumentURIClone, PositionDTO, NewNameClone).await })
	}))
}
