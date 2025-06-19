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
	/// # Returns
	/// A `Result` containing a unique handle (string) for the new WebView, or
	/// a `CommonError` on failure.
	async fn CreateWebViewPanel(
		&self,
		ExtensionData:Value, // WebViewExtensionDescriptionDTO
		ViewType:String,
		Title:String,
		ShowOptions:Value,    // WebViewShowOptionsDTO
		PanelOptions:Value,   // WebViewPanelOptionsDTO
		ContentOptions:Value, // WebViewContentOptionsDTO
	) -> Result<String, CommonError>;

	/// Disposes of a WebView panel, removing it from the UI.
	async fn DisposeWebView(&self, Handle:String) -> Result<(), CommonError>;

	/// Reveals an existing WebView panel, bringing it to the front.
	async fn RevealWebViewPanel(
		&self,
		Handle:String,
		ShowOptions:Value, // WebViewShowOptionsDTO
	) -> Result<(), CommonError>;

	/// Sets the title of a WebView panel.
	async fn SetWebViewTitle(&self, Handle:String, Title:String) -> Result<(), CommonError>;

	/// Sets the icon for a WebView panel.
	///
	/// # Parameters
	/// * `IconPath`: A DTO representing either a single `URI` or a `{ light,
	///   dark }` pair.
	async fn SetWebViewIconPath(&self, Handle:String, IconPath:Option<Value>) -> Result<(), CommonError>;

	/// Sets the HTML content of a WebView panel.
	async fn SetWebViewHTML(&self, Handle:String, HTML:String) -> Result<(), CommonError>;

	/// Posts a message from the extension host to the WebView content script.
	async fn PostMessageToWebView(&self, Handle:String, Message:Value) -> Result<bool, CommonError>;
}
