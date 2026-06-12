//! # DocumentProvider Trait
//!
//! Defines the abstract service trait for document lifecycle and modification
//! capabilities.

use async_trait::async_trait;
use serde_json::Value;
use url::Url;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// the lifecycle and content of text documents.
///
/// Implemented by `MountainEnvironment` and responsible for maintaining the
/// "single source of truth" for all open documents, handling file I/O, and
/// synchronizing state with the `Cocoon` sidecar.
#[async_trait]
pub trait DocumentProvider: Environment + Send + Sync {
	/// Opens an existing document from a URI or creates a new untitled document
	/// with initial content.
	///
	/// # Parameters
	/// * `URIComponentsDTO`: A DTO representing the URI of the document to
	///   open.
	/// * `LanguageIdentifier`: An optional language ID, typically for new
	///   documents.
	/// * `Content`: Optional initial content for a new, untitled document.
	///
	/// # Returns
	/// A `Result` containing the canonical `Url` of the opened document.
	async fn OpenDocument(
		&self,

		URIComponentsDTO:Value,

		LanguageIdentifier:Option<String>,

		Content:Option<String>,
	) -> Result<Url, CommonError>;

	/// Saves the document at the given URI to disk.
	async fn SaveDocument(&self, URI:Url) -> Result<bool, CommonError>;

	/// Saves the document currently identified by `OriginalURI` to a new
	/// location. If `NewTargetURI` is `None`, the user should be prompted to
	/// select a location.
	async fn SaveDocumentAs(&self, OriginalURI:Url, NewTargetURI:Option<Url>) -> Result<Option<Url>, CommonError>;

	/// Saves all currently "dirty" (modified) documents.
	///
	/// # Parameters
	/// * `IncludeUntitled`: If `true`, also attempts to save untitled
	///   documents, which will typically trigger a "Save As" dialog for each.
	async fn SaveAllDocuments(&self, IncludeUntitled:bool) -> Result<Vec<bool>, CommonError>;

	/// Applies a collection of content changes to the document at the given
	/// URI. This is the primary method for handling edits from the extension
	/// host.
	///
	/// # Parameters
	/// * `URI`: The URI of the document to modify.
	/// * `NewVersionIdentifier`: The new version ID of the document after the
	///   change.
	/// * `ChangesDTOCollection`: A DTO representing the set of text changes to
	///   apply.
	/// * `IsDirtyAfterChange`: A flag indicating the document's dirty state
	///   after the change.
	/// * `IsUndoing`: A flag indicating if this change is part of an "undo"
	///   operation.
	/// * `IsRedoing`: A flag indicating if this change is part of a "redo"
	///   operation.
	async fn ApplyDocumentChanges(
		&self,

		URI:Url,

		NewVersionIdentifier:i64,

		ChangesDTOCollection:Value,

		IsDirtyAfterChange:bool,

		IsUndoing:bool,

		IsRedoing:bool,
	) -> Result<(), CommonError>;
}
