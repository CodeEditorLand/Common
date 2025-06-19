//! # GetAllDiagnostics Effect
//!
//! Defines the `ActionEffect` for retrieving all diagnostics from the host.

use std::sync::Arc;

use serde_json::Value;

use super::DiagnosticManager::DiagnosticManager;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will retrieve all diagnostics
/// currently managed by the host, with an option to filter for a specific
/// resource URI.
///
/// It uses the `DiagnosticManager` capability from the environment to perform
/// the operation.
///
/// # Parameters
/// * `ResourceURIFilterOption`: An `Option<Value>` containing a serialized
///   `UriComponents` DTO. If `Some`, only diagnostics for that URI will be
///   returned. If `None`, all diagnostics for all resources are returned.
///
/// # Returns
/// An `ActionEffect` that resolves with a `serde_json::Value` representing an
/// array of `[UriComponents, MarkerDataDTO[]]` tuples.
pub fn GetAllDiagnostics<TRunTime>(
	ResourceURIFilterOption:Option<Value>,
) -> ActionEffect<Arc<TRunTime>, CommonError, Value>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn DiagnosticManager>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let FilterClone = ResourceURIFilterOption.clone();
		Box::pin(async move {
			let Manager:Arc<dyn DiagnosticManager> = RunTime.Require();
			Manager.GetAllDiagnostics(FilterClone).await
		})
	}))
}
