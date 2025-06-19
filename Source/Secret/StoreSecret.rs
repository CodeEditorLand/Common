//! # StoreSecret Effect
//!
//! Defines the `ActionEffect` for storing a secret in secure storage.

use std::sync::Arc;

use super::SecretProvider::SecretProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will store a secret in the host's
/// secure storage (e.g., OS keychain).
///
/// It uses the `SecretProvider` capability from the environment to perform the
/// actual storage operation.
///
/// # Parameters
/// * `ExtensionIdentifier`: The ID of the extension that owns the secret.
/// * `Key`: The key to store the secret under.
/// * `Value`: The secret string to be stored.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn StoreSecret<TRunTime>(
	ExtensionIdentifier:String,
	Key:String,
	Value:String,
) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn SecretProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let ExtensionIdentifierClone = ExtensionIdentifier.clone();
		let KeyClone = Key.clone();
		let ValueClone = Value.clone();
		Box::pin(async move {
			let Provider:Arc<dyn SecretProvider> = RunTime.Require();
			Provider.StoreSecret(ExtensionIdentifierClone, KeyClone, ValueClone).await
		})
	}))
}
