//! # ProvideTypeHierarchy Effect
//!
//! Defines the `ActionEffect` for requesting type hierarchy supertypes from a
//! language feature provider.

use std::sync::Arc;

use serde_json::Value;

use super::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request type hierarchy
/// supertypes.
pub fn ProvideTypeHierarchy(
	ItemDTO:Value,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Value>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let ItemDTOClone = ItemDTO.clone();

		Box::pin(async move { Registry.ProvideTypeHierarchySupertypes(ItemDTOClone).await })
	}))
}
