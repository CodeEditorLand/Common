use async_trait::async_trait;
use serde_json::Value;

/// @module WebviewProvider
/// @description Defines the abstract service trait for creating and managing
/// webviews.
use super::dto::*;
use crate::{environment::Environment, error::CommonError};

/// An abstract service contract for an environment component that can manage
/// webview panels.
///
/// This trait defines all the operations necessary for creating webview-based
/// UI, setting their content, and managing their lifecycle, abstracting away
/// the specific UI framework (e.g., Tauri, Electron) being used.
#[async_trait]
pub trait WebviewProvider: Environment + Send + Sync {
	/// Creates a new webview panel.
	/// @returns A `Result` containing a unique handle (string) for the new
	/// webview,   or a `CommonError` on failure.
	async fn CreateWebviewPanel(
		&self,
		ExtensionData:WebviewExtensionDescriptionDto,
		ViewType:String,
		Title:String,
		ShowOptions:WebviewShowOptionsDto,
		PanelOptions:WebviewPanelOptionsDto,
		ContentOptions:WebviewContentOptionsDto,
		SerializeBuffersForPostMessage:bool,
	) -> Result<String, CommonError>;

	/// Disposes of a webview panel, removing it from the UI.
	async fn DisposeWebview(&self, Handle:String) -> Result<(), CommonError>;

	/// Reveals an existing webview panel, bringing it to the front.
	async fn RevealWebviewPanel(&self, Handle:String, ShowOptions:WebviewShowOptionsDto) -> Result<(), CommonError>;

	/// Sets the title of a webview panel.
	async fn SetWebviewTitle(&self, Handle:String, Title:String) -> Result<(), CommonError>;

	/// Sets the icon for a webview panel.
	/// @param IconPath - A DTO representing either a single `Uri` or a `{
	/// light, dark }` pair.
	async fn SetWebviewIconPath(&self, Handle:String, IconPath:Option<Value>) -> Result<(), CommonError>;

	/// Sets the HTML content of a webview panel.
	async fn SetWebviewHtml(&self, Handle:String, Html:String) -> Result<(), CommonError>;

	/// Posts a message from the extension host to the webview content.
	async fn PostMessageToWebview(&self, Handle:String, Message:Value) -> Result<bool, CommonError>;
}
