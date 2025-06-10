use std::sync::Arc;

use serde_json::Value;

/// @module EstablishHostConnection
/// @description Defines a convenience ActionEffect for establishing or
/// confirming a connection to a sidecar process.
use super::SendNotificationToSidecar::SendNotificationToSidecar;
use crate::{
	effect::{ActionEffect, AppRuntime},
	error::CommonError,
};

/// Creates a convenience effect that can be used to perform an initial
/// handshake or ping a sidecar process to confirm connectivity.
///
/// This function is a wrapper around `SendNotificationToSidecar`, pre-filling
/// the method name and parameters for a standard handshake notification.
///
/// @param SidecarIdentifier - The unique ID of the sidecar process to connect
/// to.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn EstablishHostConnection<Runtime>(SidecarIdentifier:String) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static, {
	SendNotificationToSidecar(
		SidecarIdentifier,
		"$initialHandshake".to_string(), // Use the conventional '$' prefix
		Value::Null,
	)
}
