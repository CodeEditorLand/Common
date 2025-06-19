//! # SendNotificationToSidecar Effect
//!
//! Defines the `ActionEffect` for sending a fire-and-forget notification to a
//! sidecar process.

use std::sync::Arc;

use serde_json::Value;

use super::IPCProvider::IPCProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

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
pub fn SendNotificationToSidecar<TRunTime>(
	SidecarIdentifier:String,
	Method:String,
	Parameters:Value,
) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn IPCProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let SidecarIdentifierClone = SidecarIdentifier.clone();
		let MethodClone = Method.clone();
		let ParametersClone = Parameters.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Provider:Arc<dyn IPCProvider> = Environment.Require();
			Provider
				.SendNotificationToSidecar(SidecarIdentifierClone, MethodClone, ParametersClone)
				.await
		})
	}))
}
