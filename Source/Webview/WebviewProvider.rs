// File: Common/Source/Webview/WebviewProvider.rs
// Role: Defines the abstract service trait for creating and managing Webviews.
// Responsibilities:
//   - Provide a contract for creating, disposing, and revealing Webview panels.
//   - Define methods for setting a Webview's content (HTML) and options (title,

//     icon).
//   - Define a method for posting messages to a Webview's content script.

//! # WebviewProvider Trait
//!
//! Defines the abstract service trait for creating and managing Webviews.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// Webview panels.
///
/// Defines all the operations necessary for creating Webview-based UI, setting
/// their content, and managing their lifecycle, abstracting away the specific
/// UI framework (e.g., Tauri, Electron) being used by the host.
#[async_trait]
pub trait WebviewProvider: Environment + Send + Sync {
	/// Creates a new Webview panel.
	///
	/// # Parameters
	/// * `ExtensionDataValue`: DTO containing information about the extension
	///   creating the panel.
	/// * `ViewType`: A unique string identifying the type of the Webview.
	/// * `Title`: The initial title for the Webview panel.
	/// * `ShowOptionsValue`: DTO specifying the view column to show the panel
	///   in.
	/// * `PanelOptionsValue`: DTO specifying behavior options for the panel
	///   (e.g., enable scripts).
	/// * `ContentOptionsValue`: DTO specifying content options (e.g., local
	///   resource roots).
	///
	/// # Returns
	/// A `Result` containing a unique handle (string) for the new Webview, or
	/// a `CommonError` on failure.
	async fn CreateWebviewPanel(
		&self,

		// DTO: WebviewExtensionDescriptionDTO
		ExtensionDataValue:Value,

		ViewType:String,

		Title:String,

		// DTO: WebviewShowOptionsDTO
		ShowOptionsValue:Value,

		// DTO: WebviewPanelOptionsDTO
		PanelOptionsValue:Value,

		// DTO: WebviewContentOptionsDTO
		ContentOptionsValue:Value,
	) -> Result<String, CommonError>;

	/// Disposes of a Webview panel, removing it from the UI.
	///
	/// # Parameters
	/// * `Handle`: The unique handle of the Webview panel to dispose.
	async fn DisposeWebviewPanel(&self, Handle:String) -> Result<(), CommonError>;

	/// Reveals an existing Webview panel, bringing it to the front.
	///
	/// # Parameters
	/// * `Handle`: The unique handle of the Webview panel to reveal.
	/// * `ShowOptionsValue`: DTO specifying the view column to show the panel
	///   in.
	async fn RevealWebviewPanel(
		&self,

		Handle:String,

		// DTO: WebviewShowOptionsDTO
		ShowOptionsValue:Value,
	) -> Result<(), CommonError>;

	/// Sets various options for a Webview panel, such as its title and icon
	/// path.
	///
	/// # Parameters
	/// * `Handle`: The unique handle of the Webview panel to update.
	/// * `OptionsValue`: A DTO (`WebviewPanelOptionsUpdateDTO`) containing the
	///   options to set.
	async fn SetWebviewOptions(&self, Handle:String, OptionsValue:Value) -> Result<(), CommonError>;

	/// Sets the HTML content of a Webview panel.
	///
	/// # Parameters
	/// * `Handle`: The unique handle of the Webview panel.
	/// * `HTML`: The HTML string to set as the content.
	async fn SetWebviewHTML(&self, Handle:String, HTML:String) -> Result<(), CommonError>;

	/// Posts a message from the extension host to the Webview content script.
	///
	/// # Parameters
	/// * `Handle`: The unique handle of the target Webview panel.
	/// * `Message`: The JSON-serializable message to post.
	///
	/// # Returns
	/// `Ok(true)` if the message was posted successfully.
	async fn PostMessageToWebview(&self, Handle:String, Message:Value) -> Result<bool, CommonError>;
}
