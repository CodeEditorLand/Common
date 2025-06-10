use std::sync::Arc;

/// @module SaveAllDocuments
/// @description Defines the ActionEffect for saving all modified ("dirty")
/// documents.
use super::DocumentProvider::DocumentProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will save all documents that have
/// unsaved changes.
///
/// It uses the `DocumentProvider` capability from the environment to perform
/// the operation, which involves iterating through all open documents, checking
/// their dirty state, and writing the modified ones to disk.
///
/// @param IncludeUntitled - If `true`, the operation will also attempt to save
///   untitled documents, which will typically trigger a "Save As" dialog for
/// each.
///
/// @returns An `ActionEffect` that resolves with a `Vec<bool>`, where each
/// boolean   corresponds to the success of saving a particular document.
pub fn SaveAllDocuments<Runtime>(IncludeUntitled:bool) -> ActionEffect<Arc<Runtime>, CommonError, Vec<bool>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn DocumentProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn DocumentProvider> = Environment.Require();
			Provider.SaveAllDocuments(IncludeUntitled).await
		})
	}))
}
