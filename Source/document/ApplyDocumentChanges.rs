use std::sync::Arc;

use serde_json::Value;
use url::Url;

/// @module ApplyDocumentChanges
/// @description Defines the ActionEffect for applying content changes to a
/// document.
use super::DocumentProvider::DocumentProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will apply a collection of content
/// changes to the document at the given URI. This is the primary mechanism
/// for handling edits from the extension host.
///
/// It uses the `DocumentProvider` capability from the environment to perform
/// the operation, which involves updating the in-memory representation of the
/// document.
///
/// @param Uri - The `Url` of the document to modify.
/// @param NewVersionIdentifier - The new version ID of the document after the
/// change. @param ChangesDtoCollection - A DTO representing the set of text
/// changes to apply. @param IsDirtyAfterChange - A flag indicating the
/// document's dirty state after the change. @param IsUndoing - A flag
/// indicating if this change is part of an "undo" operation. @param IsRedoing -
/// A flag indicating if this change is part of a "redo" operation.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn ApplyDocumentChanges<Runtime>(
	Uri:Url,
	NewVersionIdentifier:i64,
	ChangesDtoCollection:Value,
	IsDirtyAfterChange:bool,
	IsUndoing:bool,
	IsRedoing:bool,
) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn DocumentProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let UriClone = Uri.clone();
		let ChangesClone = ChangesDtoCollection.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn DocumentProvider> = Environment.Require();
			Provider
				.ApplyDocumentChanges(
					UriClone,
					NewVersionIdentifier,
					ChangesClone,
					IsDirtyAfterChange,
					IsUndoing,
					IsRedoing,
				)
				.await
		})
	}))
}
