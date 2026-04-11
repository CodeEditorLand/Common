//! # ProvideWorkspaceSymbols Effect
//!
//! Defines the `ActionEffect` for requesting workspace symbols from a language
//! feature provider.

use std::sync::Arc;

use serde_json::Value;

use super::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will request workspace symbols.
pub fn ProvideWorkspaceSymbols(
	Query:String,
) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, Option<Value>> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		let QueryClone = Query.clone();

		Box::pin(async move { Registry.ProvideWorkspaceSymbols(QueryClone).await })
	}))
}
