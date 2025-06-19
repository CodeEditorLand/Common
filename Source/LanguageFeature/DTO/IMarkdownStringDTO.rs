//! # IMarkdownStringDTO
//!
//! Defines the Data Transfer Object for a markdown string that supports
//! trusted content and theming.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct that represents a string containing markdown. It can
/// optionally be marked as "trusted" to allow for command links and other
/// active content. This is analogous to `vscode.MarkdownString`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IMarkdownStringDTO {
	/// The markdown string content.
	pub Value:String,
	/// Whether this markdown string is trusted. Trusted strings can execute
	/// commands, for example.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IsTrusted:Option<bool>,
	/// A flag to indicate that this markdown string might contain icons that
	/// need to be rendered using a theme-aware icon font.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SupportThemeIcons:Option<bool>,
	/// A flag to indicate that this markdown string might contain HTML tags
	/// that need to be rendered.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SupportHTML:Option<bool>,
	/// An optional base URI to resolve relative paths against, especially for
	/// images. Serialized `UriComponents`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BaseURI:Option<Value>,
	/// A map of URIs that are allowed to be accessed, for sanitization
	/// purposes.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub URIs:Option<HashMap<String, Value>>,
}
