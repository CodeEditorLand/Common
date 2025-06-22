// File: Common/Source/Document/SaveDocumentAs.rs
// Role: Defines the `SaveDocumentAs` ActionEffect.
// Responsibilities:
//   - Provide a declarative effect for saving a text document to a new
//     location.
//   - This effect abstracts the "what" (save a document as) from the "how" (the
//     DocumentProvider implementation).

//! # SaveDocumentAs Effect
//!
//! Defines the `ActionEffect` for saving a document to a new location.

use std::sync::Arc;

use url::Url;

use super::DocumentProvider::DocumentProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will save a document to a new
/// location. This is typically used for "Save As..." functionality.
///
/// It uses the `DocumentProvider` capability from the environment to perform
/// the operation. If `NewTargetURI` is `None`, the provider is expected to
/// interact with the user (via the `UserInterfaceProvider`) to prompt for a
/// new save location.
///
/// # Parameters
/// * `OriginalURI`: The `Url` of the document to save.
/// * `NewTargetURI`: An optional `Url` for the new save location. If `None`, a
///   save dialog should be shown to the user.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<Url>`, containing the
/// final `Url` of the saved file or `None` if the operation was cancelled.
pub fn SaveDocumentAs(
	OriginalURI:Url,

	NewTargetURI:Option<Url>,
) -> ActionEffect<Arc<dyn DocumentProvider>, CommonError, Option<Url>> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn DocumentProvider>| {
		let OriginalURIClone = OriginalURI.clone();

		let NewTargetURIClone = NewTargetURI.clone();

		Box::pin(async move { Provider.SaveDocumentAs(OriginalURIClone, NewTargetURIClone).await })
	}))
}
