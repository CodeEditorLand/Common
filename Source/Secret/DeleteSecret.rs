//! # DeleteSecret Effect
//!
//! Defines the `ActionEffect` for deleting a secret from secure storage.

use std::sync::Arc;

use super::SecretProvider::SecretProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will delete a secret from the host's
/// secure storage (e.g., OS keychain).
/// It uses the `SecretProvider` capability from the environment to perform the
/// actual deletion.
///
/// # Parameters
/// * `ExtensionIdentifier`: The ID of the extension that owns the secret.
/// * `Key`: The key of the secret to delete.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn DeleteSecret(ExtensionIdentifier:String, Key:String) -> ActionEffect<Arc<dyn SecretProvider>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn SecretProvider>| {
		let ExtensionIdentifierClone = ExtensionIdentifier.clone();

		let KeyClone = Key.clone();

		Box::pin(async move { Provider.DeleteSecret(ExtensionIdentifierClone, KeyClone).await })
	}))
}
