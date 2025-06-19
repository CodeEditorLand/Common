//! # SaveAllDocuments Effect
//!
//! Defines the `ActionEffect` for saving all modified ("dirty") documents.

use std::sync::Arc;

use super::DocumentProvider::DocumentProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will save all documents that have
/// unsaved changes.
///
/// It uses the `DocumentProvider` capability from the environment to perform
/// the operation. This involves iterating through all open documents, checking
/// their dirty state, and writing the modified ones to disk.
///
/// # Parameters
/// * `IncludeUntitled`: If `true`, the operation will also attempt to save
///   untitled documents. This will typically trigger a "Save As..." dialog (via
///   the `UserInterfaceProvider`) for each untitled document.
///
/// # Returns
/// An `ActionEffect` that resolves with a `Vec<bool>`, where each boolean
/// corresponds to the success of saving a particular document.
pub fn SaveAllDocuments<TRunTime>(IncludeUntitled:bool) -> ActionEffect<Arc<TRunTime>, CommonError, Vec<bool>>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn DocumentProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Provider:Arc<dyn DocumentProvider> = Environment.Require();
			Provider.SaveAllDocuments(IncludeUntitled).await
		})
	}))
}
