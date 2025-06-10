use async_trait::async_trait;
use serde_json::Value;

/// @module SyncProvider
/// @description Defines the abstract service trait for synchronizing user data
/// with a remote service.
use crate::environment::Environment;
use crate::error::CommonError;

/// An abstract service contract for an environment component that can
/// synchronize user data (settings, keybindings, extensions, snippets, etc.)
/// with a remote storage backend.
#[async_trait]
pub trait SyncProvider: Environment + Send + Sync {
	/// Pushes the current user data state to the remote sync service.
	///
	/// @param UserData - A `serde_json::Value` containing the complete user
	/// data   to be synchronized.
	async fn PushUserData(&self, UserData:Value) -> Result<(), CommonError>;

	/// Pulls the latest user data state from the remote sync service.
	///
	/// @returns A `Result` containing a `serde_json::Value` with the complete
	///   user data from the remote service, which can then be applied to the
	///   local configuration.
	async fn PullUserData(&self) -> Result<Value, CommonError>;
}
