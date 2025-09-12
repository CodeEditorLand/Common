//! # SetDiagnostics Effect
//!
//! Defines the `ActionEffect` for setting or updating diagnostics for a given
//! owner.

use std::sync::Arc;

use serde_json::Value;

use super::DiagnosticManager::DiagnosticManager;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

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
pub fn SetDiagnostics(
	Owner:String,

	EntriesDTOValue:Value,
) -> ActionEffect<Arc<dyn DiagnosticManager>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Manager:Arc<dyn DiagnosticManager>| {
		let OwnerClone = Owner.clone();

		let EntriesClone = EntriesDTOValue.clone();

		Box::pin(async move { Manager.SetDiagnostics(OwnerClone, EntriesClone).await })
	}))
}
