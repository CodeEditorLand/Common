// File: Common/Source/DocumentEffect.rs
// Responsibility: Responsibility could not be determined.
// Modified: 2025-06-06 23:31:44 UTC

// Land_Common/src/document_effects.rs
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value; // For parameters and results that are flexible DTOs
use url::Url;

// Ensure AppRuntime is the correct type from your runtime module.
use crate::runtime::AppRuntime;
use crate::{
	effect::ActionEffect,
	environment::{Environment, Requires},
	errors::CommonError,
}; // For document URIs

/// Conceptual DTO for parameters often sent with document events or updates.
///
/// While not directly used as a single argument in the `DocumentProvider` trait
/// methods below, its fields represent common data associated with document
/// operations. Specific event payloads or more complex operations might use a
/// similar structure.
#[derive(Clone, Debug)]
#[allow(dead_code)] // May not be directly instantiated if fields are passed individually
pub struct DocEventParams {
	pub uri:Url,
	pub version_id:Option<i64>,
	pub language_id:Option<String>,
	pub content:Option<String>, // Full content for new/opened documents
	// Array of RpcModelContentChangeDto (or similar) for incremental updates
	pub changes:Option<Value>,
	pub eol:Option<String>,
	pub is_dirty:Option<bool>,
	pub is_undoing:Option<bool>,
	pub is_redoing:Option<bool>,
	pub new_target_uri:Option<Url>,    // For "save as" operations
	pub include_untitled:Option<bool>, // For "save all" operations
}

/// Trait for managing text documents within the application.
///
/// This includes opening, saving, and applying changes to documents,
/// abstracting the underlying storage and state management.
#[async_trait]
pub trait DocumentProvider: Environment {
	/// Opens an existing document from a URI or creates a new untitled
	/// document.
	///
	/// # Argument
	/// * `uri_components_dto`: A `serde_json::Value` representing
	///   `UriComponents`. If `Value::Null` or an empty object, a new untitled
	///   document is typically created.
	/// * `language_id`: Optional language ID to override auto-detection.
	/// * `content`: Optional initial content for the document. If provided for
	///   an existing URI, it might override the file content (behavior depends
	///   on implementation).
	///
	/// # Returns
	/// The canonical `Url` of the opened or created document.
	async fn open_document(
		&self,
		uri_components_dto:Value,
		language_id:Option<String>,
		content:Option<String>,
	) -> Result<Url, CommonError>;

	/// Saves the document at the given URI.
	/// If the document is not dirty, this might be a no-op.
	///
	/// # Returns
	/// `true` if the save was successful (or not needed), `false` if it failed
	/// or was cancelled.
	async fn save_document(&self, uri:Url) -> Result<bool, CommonError>;

	/// Saves the document currently identified by `original_uri` to a new
	/// location. If `new_target_uri` is `None`, the implementation should
	/// typically prompt the user for a new save location (e.g., via
	/// `UiProvider`).
	///
	/// # Returns
	/// `Ok(Some(Url))` with the new URI if saved successfully.
	/// `Ok(None)` if the save operation was cancelled by the user (e.g., from a
	/// dialog). `Err` if an error occurred during the save process.
	async fn save_document_as(&self, original_uri:Url, new_target_uri:Option<Url>) -> Result<Option<Url>, CommonError>;

	/// Saves all currently dirty documents.
	///
	/// # Argument
	/// * `include_untitled`: If `true`, prompts to save untitled dirty
	///   documents. If `false`, untitled documents might be skipped or handled
	///   differently.
	///
	/// # Returns
	/// A vector of booleans, where each boolean indicates the save success for
	/// a corresponding document that was attempted to be saved. The order
	/// might depend on the implementation.
	async fn save_all_documents(&self, include_untitled:bool) -> Result<Vec<bool>, CommonError>;

	/// Applies a collection of content changes to the document at the given
	/// URI.
	///
	/// # Argument
	/// * `uri`: The URI of the document to modify.
	/// * `new_version_id`: The expected version ID of the document after these
	///   changes are applied.
	/// * `changes_dto_collection`: A `serde_json::Value` representing an array
	///   of content change DTOs (e.g., `RpcModelContentChangeDto[]` from VS
	///   Code protocol).
	/// * `is_dirty_after_change`: Explicitly states if the document should be
	///   considered dirty after these changes.
	/// * `is_undoing`: Indicates if these changes are part of an "undo"
	///   operation.
	/// * `is_redoing`: Indicates if these changes are part of a "redo"
	///   operation.
	async fn apply_document_changes(
		&self,
		uri:Url,
		new_version_id:i64,
		changes_dto_collection:Value,
		is_dirty_after_change:bool,
		is_undoing:bool,
		is_redoing:bool,
	) -> Result<(), CommonError>;

	// Future considerations:
	// async fn close_document(&self, uri: Url) -> Result<(), CommonError>;
	// async fn get_document_state(&self, uri: Url) ->
	// Result<Option<DocumentStateDto>, CommonError>;
}

// --- Effect Constructors ---

/// Creates an effect to open or create a document.
pub fn try_open_document(
	uri_components_dto:Value,
	language_id:Option<String>,
	content:Option<String>,
) -> ActionEffect<Arc<AppRuntime>, CommonError, Url> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let uri_dto_clone = uri_components_dto.clone();
		let lang_id_clone = language_id.clone();
		let content_clone = content.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn DocumentProvider + Send + Sync> = concrete_env.require();
			provider.open_document(uri_dto_clone, lang_id_clone, content_clone).await
		})
	}))
}

/// Creates an effect to save a document.
pub fn try_save_document(uri:Url) -> ActionEffect<Arc<AppRuntime>, CommonError, bool> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let uri_clone = uri.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn DocumentProvider + Send + Sync> = concrete_env.require();
			provider.save_document(uri_clone).await
		})
	}))
}

/// Creates an effect to save a document to a new location ("Save As").
pub fn try_save_document_as(
	original_uri:Url,
	new_target_uri:Option<Url>,
) -> ActionEffect<Arc<AppRuntime>, CommonError, Option<Url>> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let original_uri_clone = original_uri.clone();
		let new_target_uri_clone = new_target_uri.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn DocumentProvider + Send + Sync> = concrete_env.require();
			provider.save_document_as(original_uri_clone, new_target_uri_clone).await
		})
	}))
}

/// Creates an effect to save all dirty documents.
pub fn save_all_documents(include_untitled:bool) -> ActionEffect<Arc<AppRuntime>, CommonError, Vec<bool>> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn DocumentProvider + Send + Sync> = concrete_env.require();
			provider.save_all_documents(include_untitled).await
		})
	}))
}

/// Creates an effect to apply content changes to a document.
pub fn apply_document_changes(
	uri:Url,
	new_version_id:i64,
	changes_dto_collection:Value,
	is_dirty_after_change:bool,
	is_undoing:bool,
	is_redoing:bool,
) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let uri_clone = uri.clone();
		let changes_clone = changes_dto_collection.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn DocumentProvider + Send + Sync> = concrete_env.require();
			provider
				.apply_document_changes(
					uri_clone,
					new_version_id, // i64 is Copy
					changes_clone,
					is_dirty_after_change, // bool is Copy
					is_undoing,            // bool is Copy
					is_redoing,            // bool is Copy
				)
				.await
		})
	}))
}
