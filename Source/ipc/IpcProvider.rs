use async_trait::async_trait;
use serde_json::Value;

/// @module IpcProvider
/// @description Defines the abstract service trait for inter-process
/// communication (IPC).
use crate::environment::Environment;
use crate::error::CommonError;

/// An abstract service contract for an environment component that can
/// communicate with external sidecar processes (like `Cocoon`).
///
/// This trait is implemented by `MountainEnvironment` and typically uses gRPC
/// as the underlying transport mechanism.
#[async_trait]
pub trait IpcProvider: Environment + Send + Sync {
	/// Sends a notification (a fire-and-forget message) to a specified sidecar.
	/// This method does not wait for a response.
	///
	/// @param SidecarIdentifier - The unique ID of the target sidecar process.
	/// @param Method - The name of the notification method to be invoked on the
	/// sidecar. @param Parameters - A `serde_json::Value` containing the
	/// parameters for the notification.
	async fn SendNotificationToSidecar(
		&self,
		SidecarIdentifier:String,
		Method:String,
		Parameters:Value,
	) -> Result<(), CommonError>;

	/// Sends a request to a specified sidecar and awaits a response.
	///
	/// @param SidecarIdentifier - The unique ID of the target sidecar process.
	/// @param Method - The name of the RPC method to be invoked on the sidecar.
	/// @param Parameters - A `serde_json::Value` containing the parameters for
	/// the request. @param TimeoutMilliseconds - The maximum time to wait for
	/// a response before failing.
	///
	/// @returns A `Result` containing the `serde_json::Value` response from the
	/// sidecar.
	async fn SendRequestToSidecar(
		&self,
		SidecarIdentifier:String,
		Method:String,
		Parameters:Value,
		TimeoutMilliseconds:u64,
	) -> Result<Value, CommonError>;
}
