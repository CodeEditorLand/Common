// File: Common/Source/Document/SaveDocument.rs
// Role: Defines the `SaveDocument` ActionEffect.
// Responsibilities:
//   - Provide a declarative effect for saving a single text document.
//   - This effect abstracts the "what" (save a document) from the "how" (the
//     DocumentProvider implementation).

//! # SaveDocument Effect
//!
//! Defines the `ActionEffect` for saving a single document to its persisted
//! location.

use std::sync::Arc;

use url::Url;

use super::DocumentProvider::DocumentProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will save the document at the
/// specified URI.
/// It uses the `DocumentProvider` capability from the environment to perform
/// the operation, which involves getting the document's current content from
/// the in-memory store and writing it to disk.
///
/// # Parameters
/// * `Uri`: The `Url` of the document to save.
///
/// # Returns
/// An `ActionEffect` that resolves with a `bool` indicating whether the save
/// operation was successful.
pub fn SaveDocument(Uri:Url) -> ActionEffect<Arc<dyn DocumentProvider>, CommonError, bool> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn DocumentProvider>| {
		let UriClone = Uri.clone();

		Box::pin(async move { Provider.SaveDocument(UriClone).await })
	}))
}
