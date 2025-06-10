use std::sync::Arc;

use serde_json::Value;

/// @module GetAllDiagnostics
/// @description Defines the ActionEffect for retrieving all diagnostics from
/// the host.
use super::DiagnosticsManager::DiagnosticsManager;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will retrieve all diagnostics
/// currently managed by the host, with an option to filter for a specific
/// resource URI.
///
/// It uses the `DiagnosticsManager` capability from the environment to perform
/// the operation.
///
/// @param ResourceUriFilterOption - An `Option<Value>` containing a
/// `UriComponents`   DTO. If `Some`, only diagnostics for that URI will be
/// returned. If `None`, all   diagnostics for all resources are returned.
///
/// @returns An `ActionEffect` that resolves with a `serde_json::Value`
/// representing   an array of `[UriComponents, MarkerDataDto[]]` tuples.
pub fn GetAllDiagnostics<Runtime>(
	ResourceUriFilterOption:Option<Value>,
) -> ActionEffect<Arc<Runtime>, CommonError, Value>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn DiagnosticsManager>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let FilterClone = ResourceUriFilterOption.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Manager:Arc<dyn DiagnosticsManager> = Environment.Require();
			Manager.GetAllDiagnostics(FilterClone).await
		})
	}))
}
