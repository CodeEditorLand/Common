// File: Common/Source/CustomEditor/CustomEditorProvider.rs
// Role: Defines the abstract service trait for managing custom editors.
// Responsibilities:
//   - Provide a contract for registering and unregistering custom editor
//     providers.
//   - Define the communication protocol for saving and resolving custom editor
//     content between the host (Mountain) and extension host (Cocoon).

//! # CustomEditorProvider Trait
//!
//! Defines the abstract service trait for managing custom editors.

use async_trait::async_trait;
use serde_json::Value;
use url::Url;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// the registration and resolution of custom, Webview-based editors.
#[async_trait]
pub trait CustomEditorProvider: Environment + Send + Sync {
	/// Registers a new custom editor provider from an extension.
	///
	/// # Parameters
	/// * `ViewType`: A unique identifier for the custom editor.
	/// * `OptionsValue`: A DTO containing options, including file patterns and
	///   webview options.
	async fn RegisterCustomEditorProvider(&self, ViewType:String, OptionsValue:Value) -> Result<(), CommonError>;

	/// Unregisters a previously registered custom editor provider.
	///
	/// # Parameters
	/// * `ViewType`: The identifier of the provider to unregister.
	async fn UnregisterCustomEditorProvider(&self, ViewType:String) -> Result<(), CommonError>;

	/// A notification sent from the extension host (`Cocoon`) to the main host
	/// (`Mountain`) when a custom document is saved by the user in the UI.
	///
	/// # Parameters
	/// * `ViewType`: The identifier of the custom editor.
	/// * `ResourceURI`: The URI of the document being saved.
	async fn OnSaveCustomDocument(&self, ViewType:String, ResourceURI:Url) -> Result<(), CommonError>;

	/// A request sent from the main host (`Mountain`) to the extension host
	/// (`Cocoon`) to resolve the content for a custom editor.
	///
	/// # Parameters
	/// * `ViewType`: The identifier of the custom editor.
	/// * `ResourceURI`: The URI of the document to resolve.
	/// * `WebviewPanelHandle`: The unique handle for the webview panel hosting
	///   the editor.
	async fn ResolveCustomEditor(
		&self,

		ViewType:String,

		ResourceURI:Url,

		WebviewPanelHandle:String,
	) -> Result<(), CommonError>;
}
