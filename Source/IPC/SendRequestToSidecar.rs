//! # SendRequestToSidecar Effect
//!
//! Defines the `ActionEffect` for sending a request-response RPC call to a
//! sidecar process.

use std::sync::Arc;

use serde_json::Value;

use super::IPCProvider::IPCProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will send a request to a specified
/// sidecar process and await its response.
///
/// It uses the `IPCProvider` capability from the environment to perform the
/// actual IPC request operation over the underlying transport (e.g., gRPC).
///
/// # Parameters
///
/// * `SidecarIdentifier`: The unique ID of the target sidecar process.
/// * `Method`: The name of the RPC method to be invoked on the sidecar.
/// * `Parameters`: A `serde_json::Value` containing the parameters for the
///   request.
/// * `TimeoutMilliseconds`: The maximum time to wait for a response before
///   failing.
///
/// # Returns
///
/// An `ActionEffect` that resolves with the `serde_json::Value` response from
/// the sidecar.
pub fn SendRequestToSidecar(
	SidecarIdentifier:String,

	Method:String,

	Parameters:Value,

	TimeoutMilliseconds:u64,
) -> ActionEffect<Arc<dyn IPCProvider>, CommonError, Value> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn IPCProvider>| {
		let SidecarIdentifierClone = SidecarIdentifier.clone();

		let MethodClone = Method.clone();

		let ParametersClone = Parameters.clone();

		Box::pin(async move {
			Provider
				.SendRequestToSidecar(SidecarIdentifierClone, MethodClone, ParametersClone, TimeoutMilliseconds)
				.await
		})
	}))
}
