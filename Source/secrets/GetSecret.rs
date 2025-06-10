use std::sync::Arc;

/// @module GetSecret
/// @description Defines the ActionEffect for retrieving a secret from secure
/// storage.
use super::SecretsProvider::SecretsProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will retrieve a secret from the
/// host's secure storage (e.g., OS keychain).
///
/// It uses the `SecretsProvider` capability from the environment to perform the
/// actual retrieval.
///
/// @param ExtensionIdentifier - The ID of the extension that owns the secret.
/// @param Key - The key of the secret to retrieve.
///
/// @returns An `ActionEffect` that resolves with an `Option<String>`,
/// containing the   secret's value or `None` if the secret does not exist.
pub fn GetSecret<Runtime>(
	ExtensionIdentifier:String,
	Key:String,
) -> ActionEffect<Arc<Runtime>, CommonError, Option<String>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn SecretsProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let ExtensionIdentifierClone = ExtensionIdentifier.clone();
		let KeyClone = Key.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn SecretsProvider> = Environment.Require();
			Provider.GetSecret(ExtensionIdentifierClone, KeyClone).await
		})
	}))
}
