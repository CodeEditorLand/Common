//! # SecretProvider Trait
//!
//! Defines the abstract service trait for secure storage capabilities,

//! abstracting interactions with an OS-level keychain or credential store.

use async_trait::async_trait;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that handles the
/// secure storage and retrieval of sensitive information like API tokens.
/// Implemented by `MountainEnvironment` and typically uses a library like
/// `keyring` to interact with the native operating system's credential
/// manager. Secrets are namespaced by an `ExtensionIdentifier` to ensure that
/// one extension cannot access the secrets of another.
#[async_trait]
pub trait SecretProvider: Environment + Send + Sync {
	/// Retrieves a secret for a given extension and key.
	///
	/// # Parameters
	/// * `ExtensionIdentifier`: The ID of the extension that owns the secret.
	/// * `Key`: The key identifying the secret.
	///
	/// # Returns
	/// A `Result` containing an `Option<String>`. It resolves to
	/// `Ok(Some(Value))` if the secret is found, `Ok(None)` if not found, or
	/// an `Err` on failure.
	async fn GetSecret(&self, ExtensionIdentifier:String, Key:String) -> Result<Option<String>, CommonError>;

	/// Stores a secret for a given extension and key.
	///
	/// # Parameters
	/// * `ExtensionIdentifier`: The ID of the extension that owns the secret.
	/// * `Key`: The key to store the secret under.
	/// * `Value`: The secret value to be stored.
	async fn StoreSecret(&self, ExtensionIdentifier:String, Key:String, Value:String) -> Result<(), CommonError>;

	/// Deletes a secret for a given extension and key.
	///
	/// # Parameters
	/// * `ExtensionIdentifier`: The ID of the extension that owns the secret.
	/// * `Key`: The key of the secret to delete.
	async fn DeleteSecret(&self, ExtensionIdentifier:String, Key:String) -> Result<(), CommonError>;
}
