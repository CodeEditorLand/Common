use std::sync::Arc;

/// @module DeleteSecret
/// @description Defines the ActionEffect for deleting a secret from secure
/// storage.
use super::SecretsProvider::SecretsProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will delete a secret from the host's
/// secure storage (e.g., OS keychain).
///
/// It uses the `SecretsProvider` capability from the environment to perform the
/// actual deletion.
///
/// @param ExtensionIdentifier - The ID of the extension that owns the secret.
/// @param Key - The key of the secret to delete.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn DeleteSecret<Runtime>(ExtensionIdentifier:String, Key:String) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn SecretsProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let ExtensionIdentifierClone = ExtensionIdentifier.clone();
		let KeyClone = Key.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn SecretsProvider> = Environment.Require();
			Provider.DeleteSecret(ExtensionIdentifierClone, KeyClone).await
		})
	}))
}
