//! # UnregisterProvider Effect
//!
//! Defines the `ActionEffect` for unregistering a language feature provider.

use std::sync::Arc;

use super::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will unregister a language feature
/// provider from the host's central registry using its unique handle.
///
/// It uses the `LanguageFeatureProviderRegistry` capability from the
/// environment.
///
/// # Parameters
/// * `Handle`: The `u32` handle that was returned when the provider was
///   registered.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn UnregisterProvider(Handle:u32) -> ActionEffect<Arc<dyn LanguageFeatureProviderRegistry>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Registry:Arc<dyn LanguageFeatureProviderRegistry>| {
		Box::pin(async move { Registry.UnregisterProvider(Handle).await })
	}))
}
