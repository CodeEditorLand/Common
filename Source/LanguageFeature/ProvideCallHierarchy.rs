//! # ProvideCallHierarchy Effect
//!
//! Defines the `ActionEffect` for requesting call hierarchy incoming calls from a language feature
//! provider.

use std::sync::Arc;

use serde_json::Value;

use super::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request call hierarchy incoming calls.
pub fn ProvideCallHierarchy(
	ItemDTO:Value,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Value>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let ItemDTOClone = ItemDTO.clone();

		Box::pin(async move { Registry.ProvideCallHierarchyIncomingCalls(ItemDTOClone).await })
	}))
}
