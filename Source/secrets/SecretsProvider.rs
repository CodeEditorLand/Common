use async_trait::async_trait;

/// @module SecretsProvider
/// @description Defines the abstract service trait for secure storage
/// capabilities, typically interacting with the OS keychain.
use crate::environment::Environment;
use crate::error::CommonError;

/// An abstract service contract for an environment component that handles the
/// secure storage and retrieval of secrets.
///
/// This trait is implemented by `MountainEnvironment` and uses a library like
/// `keyring` to interact with the native operating system's credential store.
/// Secrets are scoped by an `ExtensionIdentifier` to ensure that extensions
/// can only access their own secrets.
#[async_trait]
pub trait SecretsProvider: Environment + Send + Sync {
	/// Retrieves a secret for a given extension and key.
	///
	/// @param ExtensionIdentifier - The ID of the extension that owns the
	/// secret. @param Key - The key identifying the secret.
	/// @returns A `Result` containing an `Option<String>`. It resolves to
	/// `Ok(Some(Value))`   if the secret is found, `Ok(None)` if not, or an
	/// `Err` on failure.
	async fn GetSecret(&self, ExtensionIdentifier:String, Key:String) -> Result<Option<String>, CommonError>;

	/// Stores a secret for a given extension and key.
	///
	/// @param ExtensionIdentifier - The ID of the extension that owns the
	/// secret. @param Key - The key to store the secret under.
	/// @param Value - The secret value to be stored.
	async fn StoreSecret(&self, ExtensionIdentifier:String, Key:String, Value:String) -> Result<(), CommonError>;

	/// Deletes a secret for a given extension and key.
	///
	/// @param ExtensionIdentifier - The ID of the extension that owns the
	/// secret. @param Key - The key of the secret to delete.
	async fn DeleteSecret(&self, ExtensionIdentifier:String, Key:String) -> Result<(), CommonError>;
}
