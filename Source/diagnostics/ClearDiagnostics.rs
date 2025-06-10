use std::sync::Arc;

/// @module ClearDiagnostics
/// @description Defines the ActionEffect for clearing all diagnostics from a
/// specific owner.
use super::DiagnosticsManager::DiagnosticsManager;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will clear all diagnostics for a
/// given owner.
///
/// This is used when an extension is disposed of or when a linter re-runs and
/// finds no problems. It uses the `DiagnosticsManager` capability from the
/// environment to perform the operation.
///
/// @param Owner - A string identifying the source of the diagnostics to be
/// cleared   (e.g., 'typescript-linter').
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn ClearDiagnostics<Runtime>(Owner:String) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn DiagnosticsManager>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let OwnerClone = Owner.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Manager:Arc<dyn DiagnosticsManager> = Environment.Require();
			Manager.ClearDiagnostics(OwnerClone).await
		})
	}))
}
