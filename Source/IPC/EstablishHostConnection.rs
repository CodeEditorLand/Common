//! # EstablishHostConnection Effect
//!
//! Defines a convenience `ActionEffect` for establishing or confirming a
//! connection to a sidecar process.

use std::sync::Arc;

use serde_json::Value;

use super::SendNotificationToSideCar::SendNotificationToSideCar;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError, IPC::IPCProvider::IPCProvider};

/// Creates a convenience effect that can be used to perform an initial
/// handshake or ping a sidecar process to confirm connectivity.
///
/// This function is a specialized wrapper around `SendNotificationToSideCar`,

/// pre-filling the method name and parameters for a standard handshake
/// notification.
///
/// # Parameters
///
/// * `SideCarIdentifier`: The unique ID of the sidecar process to connect to.
///
/// # Returns
///
/// An `ActionEffect` that resolves to `()` on success and requires the
/// `IPCProvider` capability to be executed.
pub fn EstablishHostConnection(SideCarIdentifier:String) -> ActionEffect<Arc<dyn IPCProvider>, CommonError, ()> {
	SendNotificationToSideCar(SideCarIdentifier, "$InitialHandshake".to_string(), Value::Null)
}
