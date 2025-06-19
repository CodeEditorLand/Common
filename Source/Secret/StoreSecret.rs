//! # StoreSecret Effect
//!
//! Defines the `ActionEffect` for storing a secret in secure storage.

use std::sync::Arc;

use super::SecretProvider::SecretProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

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
pub fn StoreSecret(
	ExtensionIdentifier:String,
	Key:String,
	Value:String,
) -> ActionEffect<Arc<dyn SecretProvider>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn SecretProvider>| {
		let ExtensionIdentifierClone = ExtensionIdentifier.clone();
		let KeyClone = Key.clone();
		let ValueClone = Value.clone();
		Box::pin(async move { Provider.StoreSecret(ExtensionIdentifierClone, KeyClone, ValueClone).await })
	}))
}
