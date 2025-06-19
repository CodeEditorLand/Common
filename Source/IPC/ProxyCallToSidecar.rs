//! # ProxyCallToSidecar Effect
//!
//! Defines the `ActionEffect` for proxying a generic RPC call to a sidecar
//! process.

use std::sync::Arc;

use serde_json::Value;

use super::IPCProvider::IPCProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that proxies an RPC call to a specified target sidecar.
///
/// This is a powerful utility for scenarios where the host application
/// (`Mountain`) needs to act as a router, forwarding a request it received from
/// one process to another without needing to understand the request's content.
/// The entire call payload is encapsulated within the `CallData` object.
///
/// # Parameters
///
/// * `TargetSidecarIdentifier`: The unique ID of the sidecar to which the call
///   should be proxied.
/// * `CallData`: A JSON `Value` expected to be an object containing `{"Method":
///   "...", "Parameters": ...}`.
///
/// # Returns
///
/// An `ActionEffect` that resolves with the JSON `Value` returned by the
/// target sidecar.
pub fn ProxyCallToSidecar<TRunTime>(
	TargetSidecarIdentifier:String,
	CallData:Value,
) -> ActionEffect<Arc<TRunTime>, CommonError, Value>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn IPCProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let TargetIdentifierClone = TargetSidecarIdentifier.clone();
		let CallDataClone = CallData.clone();
		Box::pin(async move {
			let MethodString = CallDataClone
				.get("Method")
				.and_then(Value::as_str)
				.ok_or_else(|| {
					CommonError::InvalidArgument {
						ArgumentName:"CallData.Method".to_string(),
						Reason:"Expected a 'Method' string field in CallData for proxying.".to_string(),
					}
				})?
				.to_string();

			let ParametersValue = CallDataClone.get("Parameters").cloned().unwrap_or(Value::Null);

			let Environment = RunTime.GetEnvironment();
			let Provider:Arc<dyn IPCProvider> = Environment.Require();

			// Using a default timeout here; a real implementation might make this
			// configurable by extracting it from the CallData payload.
			let DefaultTimeoutMilliseconds = 30000;
			Provider
				.SendRequestToSidecar(TargetIdentifierClone, MethodString, ParametersValue, DefaultTimeoutMilliseconds)
				.await
		})
	}))
}
