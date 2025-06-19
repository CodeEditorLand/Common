//! # UnregisterProvider Effect
//!
//! Defines the `ActionEffect` for unregistering a language feature provider.

use std::sync::Arc;

use super::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

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
pub fn UnregisterProvider<TRunTime>(Handle:u32) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn LanguageFeatureProviderRegistry>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		Box::pin(async move {
			let Registry:Arc<dyn LanguageFeatureProviderRegistry> = RunTime.Require();
			Registry.UnregisterProvider(Handle).await
		})
	}))
}
