// File: Common/Source/WebView/WebViewProvider.rs
// Role: Defines the abstract service trait for creating and managing WebViews.
// Responsibilities:
//   - Provide a contract for creating, disposing, and revealing WebView panels.
//   - Define methods for setting a WebView's content (HTML) and options (title,
//     icon).
//   - Define a method for posting messages to a WebView's content script.

//! # WebViewProvider Trait
//!
//! Defines the abstract service trait for creating and managing WebViews.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// WebView panels.
///
/// This trait defines all the operations necessary for creating WebView-based
/// UI, setting their content, and managing their lifecycle, abstracting away
/// the specific UI framework (e.g., Tauri, Electron) being used by the host.
#[async_trait]
pub trait WebViewProvider: Environment + Send + Sync {
	/// Creates a new WebView panel.
	///
	/// # Parameters
	/// * `ExtensionDataValue`: DTO containing information about the extension
	///   creating the panel.
	/// * `ViewType`: A unique string identifying the type of the WebView.
	/// * `Title`: The initial title for the WebView panel.
	/// * `ShowOptionsValue`: DTO specifying the view column to show the panel
	///   in.
	/// * `PanelOptionsValue`: DTO specifying behavior options for the panel
	///   (e.g., enable scripts).
	/// * `ContentOptionsValue`: DTO specifying content options (e.g., local
	///   resource roots).
	///
	/// # Returns
	/// A `Result` containing a unique handle (string) for the new WebView, or
	/// a `CommonError` on failure.
	async fn CreateWebViewPanel(
		&self,
		// DTO: WebViewExtensionDescriptionDTO
		ExtensionDataValue:Value,
		ViewType:String,
		Title:String,
		// DTO: WebViewShowOptionsDTO
		ShowOptionsValue:Value,
		// DTO: WebViewPanelOptionsDTO
		PanelOptionsValue:Value,
		// DTO: WebViewContentOptionsDTO
		ContentOptionsValue:Value,
	) -> Result<String, CommonError>;

	/// Disposes of a WebView panel, removing it from the UI.
	///
	/// # Parameters
	/// * `Handle`: The unique handle of the WebView panel to dispose.
	async fn DisposeWebViewPanel(&self, Handle:String) -> Result<(), CommonError>;

	/// Reveals an existing WebView panel, bringing it to the front.
	///
	/// # Parameters
	/// * `Handle`: The unique handle of the WebView panel to reveal.
	/// * `ShowOptionsValue`: DTO specifying the view column to show the panel
	///   in.
	async fn RevealWebViewPanel(
		&self,
		Handle:String,
		// DTO: WebViewShowOptionsDTO
		ShowOptionsValue:Value,
	) -> Result<(), CommonError>;

	/// Sets various options for a WebView panel, such as its title and icon
	/// path.
	///
	/// # Parameters
	/// * `Handle`: The unique handle of the WebView panel to update.
	/// * `OptionsValue`: A DTO (`WebviewPanelOptionsUpdateDTO`) containing the
	///   options to set.
	async fn SetWebViewOptions(&self, Handle:String, OptionsValue:Value) -> Result<(), CommonError>;

	/// Sets the HTML content of a WebView panel.
	// # Parameters
	/// * `Handle`: The unique handle of the WebView panel.
	/// * `HTML`: The HTML string to set as the content.
	async fn SetWebViewHTML(&self, Handle:String, HTML:String) -> Result<(), CommonError>;

	/// Posts a message from the extension host to the WebView content script.
	///
	/// # Parameters
	/// * `Handle`: The unique handle of the target WebView panel.
	/// * `Message`: The JSON-serializable message to post.
	///
	/// # Returns
	/// `Ok(true)` if the message was posted successfully.
	async fn PostMessageToWebView(&self, Handle:String, Message:Value) -> Result<bool, CommonError>;
}
