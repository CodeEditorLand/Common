//! # DeleteSecret Effect
//!
//! Defines the `ActionEffect` for deleting a secret from secure storage.

use std::sync::Arc;

use super::SecretProvider::SecretProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will delete a secret from the host's
/// secure storage (e.g., OS keychain).
///
/// It uses the `SecretProvider` capability from the environment to perform the
/// actual deletion.
///
/// # Parameters
/// * `ExtensionIdentifier`: The ID of the extension that owns the secret.
/// * `Key`: The key of the secret to delete.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn DeleteSecret<TRunTime>(ExtensionIdentifier:String, Key:String) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn SecretProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let ExtensionIdentifierClone = ExtensionIdentifier.clone();
		let KeyClone = Key.clone();
		Box::pin(async move {
			let Provider:Arc<dyn SecretProvider> = RunTime.Require();
			Provider.DeleteSecret(ExtensionIdentifierClone, KeyClone).await
		})
	}))
}
