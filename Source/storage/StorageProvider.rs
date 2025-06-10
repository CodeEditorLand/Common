use async_trait::async_trait;
use serde_json::Value;

/// @module StorageProvider
/// @description Defines the abstract service trait for Memento-style persistent
/// key-value storage capabilities.
use crate::environment::Environment;
use crate::error::CommonError;

/// An abstract service contract for an environment component that provides
/// persistent storage capabilities, similar to VS Code's Memento API.
///
/// This trait is implemented by `MountainEnvironment` and is responsible for
/// reading from and writing to the appropriate JSON storage files on disk.
#[async_trait]
pub trait StorageProvider: Environment + Send + Sync {
	/// Retrieves a value from storage for a given key and scope.
	///
	/// @param IsGlobalScope - If `true`, retrieves from global storage;
	/// otherwise, from the current workspace storage. @param Key - The key of
	/// the value to retrieve.
	///
	/// @returns A `Result` containing an `Option<Value>`. It resolves to
	/// `Ok(Some(Value))`   if the key exists, `Ok(None)` if it does not, or an
	/// `Err` on failure (e.g., I/O error).
	async fn GetStorageValue(&self, IsGlobalScope:bool, Key:&str) -> Result<Option<Value>, CommonError>;

	/// Updates or stores a value in storage.
	///
	/// @param IsGlobalScope - If `true`, updates global storage; otherwise,
	/// workspace storage. @param Key - The key of the value to update.
	/// @param ValueToSet - The `serde_json::Value` to store. If this is `None`,
	/// the key   should be deleted from storage.
	async fn UpdateStorageValue(
		&self,
		IsGlobalScope:bool,
		Key:String,
		ValueToSet:Option<Value>,
	) -> Result<(), CommonError>;
}
