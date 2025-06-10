/// @module WebviewContentOptionsDto
/// @description Defines the Data Transfer Object for a webview's content and
/// security settings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct that represents the options controlling the content
/// within a webview, including script enablement and local resource access.
///
/// This DTO is sent from Cocoon to Mountain when a webview is created.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WebviewContentOptionsDto {
	/// Enables the use of `vscode:command:` URIs within the webview.
	pub EnableCommandUris:Option<bool>,

	/// Enables the execution of scripts within the webview.
	pub EnableScripts:Option<bool>,

	/// Enables the use of HTML forms within the webview.
	pub EnableForms:Option<bool>,

	/// An optional array of port mappings for forwarding traffic from the
	/// webview to the extension host.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PortMapping:Option<Value>, // DTO: Vec<{ webviewPort: number, extensionHostPort: number }>

	/// An optional array of URIs that define the root paths from which the
	/// webview is allowed to load local resources.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LocalResourceRoots:Option<Value>, // DTO: Vec<UriComponents>
}
