use std::sync::Arc;

use serde_json::Value;

/// @module ProxyCallToSidecar
/// @description Defines the ActionEffect for proxying a generic RPC call to a
/// sidecar process.
use super::IpcProvider::IpcProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that proxies an RPC call to a specified target sidecar.
///
/// This is a powerful utility for scenarios where the host application
/// (`Mountain`) needs to act as a router, forwarding a request it received from
/// one process to another. The entire call payload is encapsulated within the
/// `CallData` object.
///
/// @param TargetSidecarIdentifier - The unique ID of the sidecar to which the
/// call should be proxied. @param CallData - A JSON `Value` expected to be an
/// object containing `{"Method": "...", "Parameters": ...}`.
///
/// @returns An `ActionEffect` that resolves with the JSON `Value` returned by
/// the target sidecar.
pub fn ProxyCallToSidecar<Runtime>(
	TargetSidecarIdentifier:String,
	CallData:Value,
) -> ActionEffect<Arc<Runtime>, CommonError, Value>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn IpcProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let TargetIdentifierClone = TargetSidecarIdentifier.clone();
		let CallDataClone = CallData.clone();
		Box::pin(async move {
			let MethodString = CallDataClone
				.get("Method")
				.and_then(Value::as_str)
				.ok_or_else(|| {
					CommonError::InvalidArg {
						ArgumentName:"CallData.Method".to_string(),
						Reason:"Expected a 'Method' string field in CallData for proxying.".to_string(),
					}
				})?
				.to_string();

			let ParametersValue = CallDataClone.get("Parameters").cloned().unwrap_or(Value::Null);
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn IpcProvider> = Environment.Require();

			// Using a default timeout here; a real implementation might make this
			// configurable.
			let DefaultTimeoutMilliseconds = 30000;
			Provider
				.SendRequestToSidecar(TargetIdentifierClone, MethodString, ParametersValue, DefaultTimeoutMilliseconds)
				.await
		})
	}))
}
