//! # StorageProvider Trait
//!
//! Defines the abstract service trait for Memento-style persistent key-value
//! storage capabilities.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that provides
/// persistent key-value storage, similar to VS Code's Memento API.
///
/// This trait is implemented by `MountainEnvironment` and is responsible for
/// reading from and writing to the appropriate JSON storage files on disk,

/// separating global state from workspace-specific state.
#[async_trait]
pub trait StorageProvider: Environment + Send + Sync {
	/// Retrieves a value from storage for a given key and scope.
	///
	/// # Parameters
	/// * `IsGlobalScope`: If `true`, retrieves from global storage; otherwise,

	///   retrieves from the current workspace's storage.
	/// * `Key`: The key of the value to retrieve.
	///
	/// # Returns
	/// A `Result` containing an `Option<Value>`. It resolves to
	/// `Ok(Some(Value))` if the key exists, `Ok(None)` if it does not, or an
	/// `Err` on failure (e.g., I/O error).
	async fn GetStorageValue(&self, IsGlobalScope:bool, Key:&str) -> Result<Option<Value>, CommonError>;

	/// Updates or stores a value in storage for a given key and scope.
	///
	/// # Parameters
	/// * `IsGlobalScope`: If `true`, updates global storage; otherwise,

	///   workspace storage.
	/// * `Key`: The key of the value to update.
	/// * `ValueToSet`: The `serde_json::Value` to store. If this is `None`, the
	///   key should be deleted from storage.
	async fn UpdateStorageValue(
		&self,

		IsGlobalScope:bool,

		Key:String,

		ValueToSet:Option<Value>,
	) -> Result<(), CommonError>;

	/// Retrieves the entire storage state for a given scope.
	async fn GetAllStorage(&self, IsGlobalScope:bool) -> Result<Value, CommonError>;

	/// Overwrites the entire storage state for a given scope with a new state.
	async fn SetAllStorage(&self, IsGlobalScope:bool, FullState:Value) -> Result<(), CommonError>;
}
