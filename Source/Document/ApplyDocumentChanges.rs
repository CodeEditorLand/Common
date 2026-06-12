//! # ApplyDocumentChanges Effect
//!
//! Defines the `ActionEffect` for applying a collection of text content
//! changes to a document.

use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::DocumentProvider::DocumentProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will apply a collection of content
/// changes to the document at the given URI. This is the primary mechanism
/// for handling edits originating from the extension host.
/// It uses the `DocumentProvider` capability from the environment to perform
/// the operation, which involves updating the in-memory representation of the
/// document's text.
///
/// # Parameters
/// * `URI`: The `Url` of the document to modify.
/// * `NewVersionIdentifier`: The new version ID of the document after the
///   change is applied.
/// * `ChangesDTOCollection`: A DTO representing the set of text changes to
///   apply.
/// * `IsDirtyAfterChange`: A flag indicating the document's dirty state after
///   the change.
/// * `IsUndoing`: A flag indicating if this change is part of an "undo"
///   operation.
/// * `IsRedoing`: A flag indicating if this change is part of a "redo"
///   operation.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn ApplyDocumentChanges(
	URI:Url,

	NewVersionIdentifier:i64,

	ChangesDTOCollection:Value,

	IsDirtyAfterChange:bool,

	IsUndoing:bool,

	IsRedoing:bool,
) -> ActionEffect<Arc<dyn DocumentProvider>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn DocumentProvider>| {
		let URIClone = URI.clone();

		let ChangesClone = ChangesDTOCollection.clone();

		Box::pin(async move {
			Provider
				.ApplyDocumentChanges(
					URIClone,
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
