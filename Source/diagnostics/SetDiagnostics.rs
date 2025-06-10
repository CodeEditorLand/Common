use std::sync::Arc;

use serde_json::Value;

/// @module SetDiagnostics
/// @description Defines the ActionEffect for setting or updating diagnostics
/// for a given owner.
use super::DiagnosticsManager::DiagnosticsManager;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will set or update diagnostics for a
/// given owner. This is the primary way extensions report problems.
///
/// It uses the `DiagnosticsManager` capability from the environment to perform
/// the operation, which involves updating the central diagnostic store.
///
/// @param Owner - A string identifying the source of the diagnostics (e.g.,
/// 'typescript-linter'). @param EntriesDtoValue - A `serde_json::Value`
/// representing an array of entries.   Each entry is a tuple:
/// `[UriComponentsValue, Option<Vec<MarkerDataDtoAsValue>>]`.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn SetDiagnostics<Runtime>(Owner:String, EntriesDtoValue:Value) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn DiagnosticsManager>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let OwnerClone = Owner.clone();
		let EntriesClone = EntriesDtoValue.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Manager:Arc<dyn DiagnosticsManager> = Environment.Require();
			Manager.SetDiagnostics(OwnerClone, EntriesClone).await
		})
	}))
}
