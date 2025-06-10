use std::sync::Arc;

use serde_json::Value;

/// @module SendNotificationToSidecar
/// @description Defines the ActionEffect for sending a fire-and-forget
/// notification to a sidecar process.
use super::IpcProvider::IpcProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will send a fire-and-forget
/// notification to a specified sidecar process.
///
/// It uses the `IpcProvider` capability from the environment to perform the
/// actual IPC send operation.
///
/// @param SidecarIdentifier - The unique ID of the target sidecar process.
/// @param Method - The name of the notification method to be invoked on the
/// sidecar. @param Parameters - A `serde_json::Value` containing the parameters
/// for the notification.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn SendNotificationToSidecar<Runtime>(
	SidecarIdentifier:String,
	Method:String,
	Parameters:Value,
) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn IpcProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let SidecarIdentifierClone = SidecarIdentifier.clone();
		let MethodClone = Method.clone();
		let ParametersClone = Parameters.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn IpcProvider> = Environment.Require();
			Provider
				.SendNotificationToSidecar(SidecarIdentifierClone, MethodClone, ParametersClone)
				.await
		})
	}))
}
