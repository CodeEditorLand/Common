//! # EstablishHostConnection Effect
//!
//! Defines a convenience `ActionEffect` for establishing or confirming a
//! connection to a sidecar process.

use std::sync::Arc;

use serde_json::Value;

use super::SendNotificationToSidecar::SendNotificationToSidecar;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Error::CommonError::CommonError,
};

/// Creates a convenience effect that can be used to perform an initial
/// handshake or ping a sidecar process to confirm connectivity.
///
/// This function is a specialized wrapper around `SendNotificationToSidecar`,
/// pre-filling the method name and parameters for a standard handshake
/// notification.
///
/// # Parameters
///
/// * `SidecarIdentifier`: The unique ID of the sidecar process to connect to.
///
/// # Returns
///
/// An `ActionEffect` that resolves to `()` on success.
pub fn EstablishHostConnection<TRunTime>(SidecarIdentifier:String) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static, {
	SendNotificationToSidecar(
		SidecarIdentifier,
		"$InitialHandshake".to_string(), // Use the conventional '$' prefix
		Value::Null,
	)
}
