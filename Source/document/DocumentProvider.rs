use async_trait::async_trait;
use serde_json::Value;
use url::Url;

/// @module DocumentProvider
/// @description Defines the abstract service trait for document lifecycle and
/// modification capabilities.
use crate::environment::Environment;
use crate::error::CommonError;

/// An abstract service contract for an environment component that can manage
/// the lifecycle and content of text documents.
///
/// This trait is implemented by `MountainEnvironment` and is responsible for
/// maintaining the "single source of truth" for all open documents, handling
/// file I/O, and synchronizing state with the `Cocoon` sidecar.
#[async_trait]
pub trait DocumentProvider: Environment + Send + Sync {
	/// Opens an existing document from a URI or creates a new untitled document
	/// with initial content.
	///
	/// @param UriComponentsDto - A DTO representing the URI of the document to
	/// open. @param LanguageIdentifier - An optional language ID, typically
	/// for new documents. @param Content - Optional initial content for a new,
	/// untitled document. @returns A `Result` containing the canonical `Url`
	/// of the opened document.
	async fn OpenDocument(
		&self,
		UriComponentsDto:Value,
		LanguageIdentifier:Option<String>,
		Content:Option<String>,
	) -> Result<Url, CommonError>;

	/// Saves the document at the given URI to disk.
	async fn SaveDocument(&self, Uri:Url) -> Result<bool, CommonError>;

	/// Saves the document currently identified by `OriginalUri` to a new
	/// location. If `NewTargetUri` is `None`, the user should be prompted to
	/// select a location.
	async fn SaveDocumentAs(&self, OriginalUri:Url, NewTargetUri:Option<Url>) -> Result<Option<Url>, CommonError>;

	/// Saves all currently "dirty" (modified) documents.
	///
	/// @param IncludeUntitled - If `true`, also attempts to save untitled
	/// documents,   which will typically trigger a "Save As" dialog for each.
	async fn SaveAllDocuments(&self, IncludeUntitled:bool) -> Result<Vec<bool>, CommonError>;

	/// Applies a collection of content changes to the document at the given
	/// URI. This is the primary method for handling edits from the extension
	/// host.
	///
	/// @param Uri - The URI of the document to modify.
	/// @param NewVersionIdentifier - The new version ID of the document after
	/// the change. @param ChangesDtoCollection - A DTO representing the set of
	/// text changes to apply. @param IsDirtyAfterChange - A flag indicating
	/// the document's dirty state after the change. @param IsUndoing - A flag
	/// indicating if this change is part of an "undo" operation.
	/// @param IsRedoing - A flag indicating if this change is part of a "redo"
	/// operation.
	async fn ApplyDocumentChanges(
		&self,
		Uri:Url,
		NewVersionIdentifier:i64,
		ChangesDtoCollection:Value,
		IsDirtyAfterChange:bool,
		IsUndoing:bool,
		IsRedoing:bool,
	) -> Result<(), CommonError>;
}
