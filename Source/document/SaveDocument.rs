use std::sync::Arc;

use url::Url;

/// @module SaveDocument
/// @description Defines the ActionEffect for saving a single document.
use super::DocumentProvider::DocumentProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will save the document at the
/// specified URI.
///
/// It uses the `DocumentProvider` capability from the environment to perform
/// the operation, which involves getting the document's content from the
/// in-memory store and writing it to disk.
///
/// @param Uri - The `Url` of the document to save.
///
/// @returns An `ActionEffect` that resolves with a `bool` indicating whether
/// the   save operation was successful.
pub fn SaveDocument<Runtime>(Uri:Url) -> ActionEffect<Arc<Runtime>, CommonError, bool>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn DocumentProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let UriClone = Uri.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn DocumentProvider> = Environment.Require();
			Provider.SaveDocument(UriClone).await
		})
	}))
}
