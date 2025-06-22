// File: Common/Source/Document/OpenDocument.rs
// Role: Defines the `OpenDocument` ActionEffect.
// Responsibilities:
//   - Provide a declarative effect for opening or creating a new text document.
//   - This effect abstracts the "what" (open a document) from the "how" (the
//     DocumentProvider implementation).

//! # OpenDocument Effect
//!
//! Defines the `ActionEffect` for opening or creating a new text document.

use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::DocumentProvider::DocumentProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will open an existing document from a
/// URI or create a new untitled document, potentially with initial content.
///
/// It uses the `DocumentProvider` capability from the environment to perform
/// the operation, which may involve file I/O and updating the central document
/// store.
///
/// # Parameters
///
/// * `URIComponentsDTO`: A `serde_json::Value` DTO representing the URI of the
///   document to open, or `null` for a new untitled document.
/// * `LanguageIdentifier`: An optional language ID, primarily for new untitled
///   documents.
/// * `Content`: Optional initial content for a new untitled document.
///
/// # Returns
///
/// An `ActionEffect` that resolves with the canonical `Url` of the opened
/// document.
pub fn OpenDocument(
	URIComponentsDTO:Value,

	LanguageIdentifier:Option<String>,

	Content:Option<String>,
) -> ActionEffect<Arc<dyn DocumentProvider>, CommonError, Url> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn DocumentProvider>| {
		let URIDTOClone = URIComponentsDTO.clone();

		let LanguageIdentifierClone = LanguageIdentifier.clone();

		let ContentClone = Content.clone();

		Box::pin(async move { Provider.OpenDocument(URIDTOClone, LanguageIdentifierClone, ContentClone).await })
	}))
}
