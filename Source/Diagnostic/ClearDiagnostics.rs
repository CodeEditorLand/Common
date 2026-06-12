//! # ClearDiagnostics Effect
//!
//! Defines the `ActionEffect` for clearing all diagnostics from a specific
//! owner.

use std::sync::Arc;

use super::DiagnosticManager::DiagnosticManager;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will clear all diagnostics for a
/// given owner.
/// This is used when an extension is disposed of or when a linter re-runs and
/// finds no problems. It uses the `DiagnosticManager` capability from the
/// environment to perform the operation.
///
/// # Parameters
/// * `Owner`: A string identifying the source of the diagnostics to be cleared
///   (e.g., 'typescript-linter').
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn ClearDiagnostics(Owner:String) -> ActionEffect<Arc<dyn DiagnosticManager>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Manager:Arc<dyn DiagnosticManager>| {
		let OwnerClone = Owner.clone();

		Box::pin(async move { Manager.ClearDiagnostics(OwnerClone).await })
	}))
}
