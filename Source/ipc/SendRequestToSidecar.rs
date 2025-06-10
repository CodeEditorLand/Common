use std::sync::Arc;

use serde_json::Value;

/// @module SendRequestToSidecar
/// @description Defines the ActionEffect for sending a request-response RPC
/// call to a sidecar process.
use super::IpcProvider::IpcProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will send a request to a specified
/// sidecar process and await its response.
///
/// It uses the `IpcProvider` capability from the environment to perform the
/// actual IPC request operation.
///
/// @param SidecarIdentifier - The unique ID of the target sidecar process.
/// @param Method - The name of the RPC method to be invoked on the sidecar.
/// @param Parameters - A `serde_json::Value` containing the parameters for the
/// request. @param TimeoutMilliseconds - The maximum time to wait for a
/// response before failing.
///
/// @returns An `ActionEffect` that resolves with the `serde_json::Value`
/// response from the sidecar.
pub fn SendRequestToSidecar<Runtime>(
	SidecarIdentifier:String,
	Method:String,
	Parameters:Value,
	TimeoutMilliseconds:u64,
) -> ActionEffect<Arc<Runtime>, CommonError, Value>
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
				.SendRequestToSidecar(SidecarIdentifierClone, MethodClone, ParametersClone, TimeoutMilliseconds)
				.await
		})
	}))
}
