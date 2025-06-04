// Land_Common/src/secrets_effects.rs
use std::sync::Arc;

use async_trait::async_trait;

// Ensure AppRuntime is the correct type from your runtime module.
use crate::runtime::AppRuntime;
use crate::{
	effect::ActionEffect,
	environment::{Environment, Requires},
	errors::CommonError,
};

/// Trait for an environment component that handles secure storage and retrieval
/// of secrets.
///
/// Secrets are typically scoped by an `extension_id` to ensure that extensions
/// can only access their own secrets.
#[async_trait]
pub trait SecretsProvider: Environment {
	/// Retrieves a secret for a given extension and key.
	/// Returns `Ok(None)` if the secret is not found.
	async fn get_secret(&self, extension_id:String, key:String) -> Result<Option<String>, CommonError>;

	/// Stores a secret for a given extension and key.
	/// This will typically overwrite any existing secret with the same
	/// extension ID and key.
	async fn store_secret(&self, extension_id:String, key:String, value:String) -> Result<(), CommonError>;

	/// Deletes a secret for a given extension and key.
	/// Should succeed even if the secret does not exist (idempotent).
	async fn delete_secret(&self, extension_id:String, key:String) -> Result<(), CommonError>;
}

// --- Effect Constructors ---

/// Creates an effect to retrieve a secret.
pub fn get_secret(extension_id:String, key:String) -> ActionEffect<Arc<AppRuntime>, CommonError, Option<String>> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let eid_clone = extension_id.clone();
		let key_clone = key.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn SecretsProvider + Send + Sync> = concrete_env.require();
			provider.get_secret(eid_clone, key_clone).await
		})
	}))
}

/// Creates an effect to store a secret.
pub fn store_secret(extension_id:String, key:String, value:String) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let eid_clone = extension_id.clone();
		let key_clone = key.clone();
		let val_clone = value.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn SecretsProvider + Send + Sync> = concrete_env.require();
			provider.store_secret(eid_clone, key_clone, val_clone).await
		})
	}))
}

/// Creates an effect to delete a secret.
pub fn delete_secret(extension_id:String, key:String) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let eid_clone = extension_id.clone();
		let key_clone = key.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn SecretsProvider + Send + Sync> = concrete_env.require();
			provider.delete_secret(eid_clone, key_clone).await
		})
	}))
}
