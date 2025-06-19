//! # GetSecret Effect
//!
//! Defines the `ActionEffect` for retrieving a secret from secure storage.

use std::sync::Arc;

use super::SecretProvider::SecretProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will retrieve a secret from the
/// host's secure storage (e.g., OS keychain).
///
/// It uses the `SecretProvider` capability from the environment to perform the
/// actual retrieval.
///
/// # Parameters
/// * `ExtensionIdentifier`: The ID of the extension that owns the secret.
/// * `Key`: The key of the secret to retrieve.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<String>`, containing the
/// secret's value or `None` if the secret does not exist.
pub fn GetSecret(
	ExtensionIdentifier:String,
	Key:String,
) -> ActionEffect<Arc<dyn SecretProvider>, CommonError, Option<String>> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn SecretProvider>| {
		let ExtensionIdentifierClone = ExtensionIdentifier.clone();
		let KeyClone = Key.clone();
		Box::pin(async move { Provider.GetSecret(ExtensionIdentifierClone, KeyClone).await })
	}))
}
