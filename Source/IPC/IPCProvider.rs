//! # IPCProvider Trait
//!
//! Defines the abstract service trait for inter-process communication (IPC)
//! capabilities.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can
/// communicate with external sidecar processes (like `Cocoon`).
///
/// Implemented by `MountainEnvironment` and typically uses gRPC as the
/// underlying transport mechanism to send and receive messages.
#[async_trait]
pub trait IPCProvider: Environment + Send + Sync {
	/// Sends a notification (a fire-and-forget message) to a specified
	/// sidecar. This method does not wait for a response.
	///
	/// # Parameters
	/// * `SideCarIdentifier`: The unique ID of the target sidecar process.
	/// * `Method`: The name of the notification method to be invoked on the
	///   sidecar.
	/// * `Parameters`: A `serde_json::Value` containing the parameters for the
	///   notification.
	async fn SendNotificationToSideCar(
		&self,

		SideCarIdentifier:String,

		Method:String,

		Parameters:Value,
	) -> Result<(), CommonError>;

	/// Sends a request to a specified sidecar and awaits a response.
	///
	/// # Parameters
	/// * `SideCarIdentifier`: The unique ID of the target sidecar process.
	/// * `Method`: The name of the RPC method to be invoked on the sidecar.
	/// * `Parameters`: A `serde_json::Value` containing the parameters for the
	///   request.
	/// * `TimeoutMilliseconds`: The maximum time to wait for a response before
	///   failing.
	///
	/// # Returns
	/// A `Result` containing the `serde_json::Value` response from the
	/// sidecar.
	async fn SendRequestToSideCar(
		&self,

		SideCarIdentifier:String,

		Method:String,

		Parameters:Value,

		TimeoutMilliseconds:u64,
	) -> Result<Value, CommonError>;
}
