//! # SendNotificationToSidecar Effect
//!
//! Defines the `ActionEffect` for sending a fire-and-forget notification to a
//! sidecar process.

use std::sync::Arc;

use serde_json::Value;

use super::IPCProvider::IPCProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will send a fire-and-forget
/// notification to a specified sidecar process.
///
/// It uses the `IPCProvider` capability from the environment to perform the
/// actual IPC send operation. Unlike `SendRequestToSidecar`, this effect does
/// not wait for or expect a response.
///
/// # Parameters
///
/// * `SidecarIdentifier`: The unique ID of the target sidecar process.
/// * `Method`: The name of the notification method to be invoked on the
///   sidecar.
/// * `Parameters`: A `serde_json::Value` containing the parameters for the
///   notification.
///
/// # Returns
///
/// An `ActionEffect` that resolves to `()` on success.
pub fn SendNotificationToSidecar(
	SidecarIdentifier:String,
	Method:String,
	Parameters:Value,
) -> ActionEffect<Arc<dyn IPCProvider>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn IPCProvider>| {
		let SidecarIdentifierClone = SidecarIdentifier.clone();
		let MethodClone = Method.clone();
		let ParametersClone = Parameters.clone();
		Box::pin(async move {
			Provider
				.SendNotificationToSidecar(SidecarIdentifierClone, MethodClone, ParametersClone)
				.await
		})
	}))
}
