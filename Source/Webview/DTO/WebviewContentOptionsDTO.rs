//! # WebviewContentOptionsDTO
//!
//! Defines the Data Transfer Object for a Webview's content and security
//! settings.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct that represents the options controlling the content
/// within a Webview, including script enablement and local resource access.
/// This DTO is sent from `Cocoon` to `Mountain` when a Webview is created to
/// configure its security sandbox and capabilities.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebviewContentOptionsDTO {
	/// Enables the use of `vscode:command:` URIs within the Webview.
	#[serde(rename = "enableCommandUris")]
	pub EnableCommandURIs:Option<bool>,

	/// Enables the execution of scripts within the Webview.
	pub EnableScripts:Option<bool>,

	/// Enables the use of HTML forms within the Webview.
	pub EnableForms:Option<bool>,

	/// An optional array of port mappings for forwarding traffic from the
	/// Webview to the extension host. Serialized
	/// `Vec<{ webviewPort: number, extensionHostPort: number }>`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PortMapping:Option<Value>,

	/// An optional array of URIs that define the root paths from which the
	/// Webview is allowed to load local resources. Serialized
	/// `Vec<UriComponents>`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LocalResourceRoots:Option<Value>,
}
