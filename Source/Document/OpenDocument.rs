//! # OpenDocument Effect
//!
//! Defines the `ActionEffect` for opening or creating a new text document.

use std::sync::Arc;

use serde_json::Value;
use url::Url;

use super::DocumentProvider::DocumentProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

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
pub fn OpenDocument<TRunTime>(
	URIComponentsDTO:Value,
	LanguageIdentifier:Option<String>,
	Content:Option<String>,
) -> ActionEffect<Arc<TRunTime>, CommonError, Url>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn DocumentProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let URIDTOClone = URIComponentsDTO.clone();
		let LanguageIdentifierClone = LanguageIdentifier.clone();
		let ContentClone = Content.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Provider:Arc<dyn DocumentProvider> = Environment.Require();
			Provider.OpenDocument(URIDTOClone, LanguageIdentifierClone, ContentClone).await
		})
	}))
}
