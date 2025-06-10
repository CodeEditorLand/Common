use std::sync::Arc;

use serde_json::Value;
use url::Url;

/// @module OpenDocument
/// @description Defines the ActionEffect for opening or creating a text
/// document.
use super::DocumentProvider::DocumentProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will open an existing document from a
/// URI or create a new untitled document, potentially with initial content.
///
/// It uses the `DocumentProvider` capability from the environment to perform
/// the operation, which may involve file I/O and updating the central document
/// store.
///
/// @param UriComponentsDto - A `serde_json::Value` DTO representing the URI of
/// the   document to open, or `null` for a new untitled document.
/// @param LanguageIdentifier - An optional language ID, primarily for new
/// untitled documents. @param Content - Optional initial content for a new
/// untitled document.
///
/// @returns An `ActionEffect` that resolves with the canonical `Url` of the
/// opened document.
pub fn OpenDocument<Runtime>(
	UriComponentsDto:Value,
	LanguageIdentifier:Option<String>,
	Content:Option<String>,
) -> ActionEffect<Arc<Runtime>, CommonError, Url>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn DocumentProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let UriDtoClone = UriComponentsDto.clone();
		let LangIdClone = LanguageIdentifier.clone();
		let ContentClone = Content.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn DocumentProvider> = Environment.Require();
			Provider.OpenDocument(UriDtoClone, LangIdClone, ContentClone).await
		})
	}))
}
