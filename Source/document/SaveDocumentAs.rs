use std::sync::Arc;

use url::Url;

/// @module SaveDocumentAs
/// @description Defines the ActionEffect for saving a document to a new
/// location.
use super::DocumentProvider::DocumentProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will save a document to a new
/// location. This is typically used for "Save As..." functionality.
///
/// It uses the `DocumentProvider` capability from the environment to perform
/// the operation. If `NewTargetUri` is `None`, the provider is expected to
/// prompt the user for a new save location.
///
/// @param OriginalUri - The `Url` of the document to save.
/// @param NewTargetUri - An optional `Url` for the new save location.
///
/// @returns An `ActionEffect` that resolves with an `Option<Url>`, containing
/// the   final `Url` of the saved file or `None` if the operation was
/// cancelled.
pub fn SaveDocumentAs<Runtime>(
	OriginalUri:Url,
	NewTargetUri:Option<Url>,
) -> ActionEffect<Arc<Runtime>, CommonError, Option<Url>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn DocumentProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let OriginalUriClone = OriginalUri.clone();
		let NewTargetUriClone = NewTargetUri.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn DocumentProvider> = Environment.Require();
			Provider.SaveDocumentAs(OriginalUriClone, NewTargetUriClone).await
		})
	}))
}
