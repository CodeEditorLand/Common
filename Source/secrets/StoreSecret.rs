use std::sync::Arc;

/// @module StoreSecret
/// @description Defines the ActionEffect for storing a secret in secure
/// storage.
use super::SecretsProvider::SecretsProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will store a secret in the host's
/// secure storage (e.g., OS keychain).
///
/// It uses the `SecretsProvider` capability from the environment to perform the
/// actual storage operation.
///
/// @param ExtensionIdentifier - The ID of the extension that owns the secret.
/// @param Key - The key to store the secret under.
/// @param Value - The secret string to be stored.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn StoreSecret<Runtime>(
	ExtensionIdentifier:String,
	Key:String,
	Value:String,
) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn SecretsProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let ExtensionIdentifierClone = ExtensionIdentifier.clone();
		let KeyClone = Key.clone();
		let ValueClone = Value.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn SecretsProvider> = Environment.Require();
			Provider.StoreSecret(ExtensionIdentifierClone, KeyClone, ValueClone).await
		})
	}))
}
