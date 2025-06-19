//! # SetDiagnostics Effect
//!
//! Defines the `ActionEffect` for setting or updating diagnostics for a given
//! owner.

use std::sync::Arc;

use serde_json::Value;

use super::DiagnosticManager::DiagnosticManager;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will set or update diagnostics for a
/// given owner. This is the primary way extensions report problems to the host.
///
/// It uses the `DiagnosticManager` capability from the environment to perform
/// the operation, which involves updating the central diagnostic store.
///
/// # Parameters
/// * `Owner`: A string identifying the source of the diagnostics (e.g.,
///   'typescript-linter').
/// * `EntriesDTOValue`: A `serde_json::Value` representing an array of entries.
///   Each entry is a tuple: `[UriComponentsValue,
///   Option<Vec<MarkerDataDTOAsValue>>]`.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn SetDiagnostics<TRunTime>(Owner:String, EntriesDTOValue:Value) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn DiagnosticManager>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let OwnerClone = Owner.clone();
		let EntriesClone = EntriesDTOValue.clone();
		Box::pin(async move {
			let Manager:Arc<dyn DiagnosticManager> = RunTime.Require();
			Manager.SetDiagnostics(OwnerClone, EntriesClone).await
		})
	}))
}
