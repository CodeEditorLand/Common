//! # StatusBarEntryDTO
//!
//! Defines the Data Transfer Object for a single status bar item.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct that represents the complete state of a single status
/// bar item, analogous to `vscode.StatusBarItem`.
///
/// This DTO is sent from the `Cocoon` sidecar to the `Mountain` host whenever
/// an extension creates or updates a status bar item, providing the host with
/// all the information needed to render it in the UI.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StatusBarEntryDTO {
	/// An internal, host-generated unique ID for this entry instance.
	pub EntryIdentifier:String,

	/// The identifier of the status bar item, as provided by the extension.
	pub ItemIdentifier:String,

	/// The identifier of the extension that owns this status bar item.
	pub ExtensionIdentifier:String,

	/// An optional name for the status bar item, used for identification.
	pub Name:String,

	/// The text to be displayed for this item (can include icons like
	/// `$(icon)`).
	pub Text:String,

	/// The tooltip to show when hovering over the item. Can be a simple string
	/// or a complex `IMarkdownStringDTO`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Tooltip:Option<Value>,

	/// A flag indicating if the extension host has a dynamic tooltip provider
	/// for this item, requiring a reverse RPC call to resolve.
	pub HasTooltipProvider:bool,

	/// The command to execute when the item is clicked. Serialized
	/// `CommandDTO`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Command:Option<Value>,

	/// The foreground color for this item. Serialized `string | ThemeColor`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Color:Option<Value>,

	/// The background color for this item. Serialized `ThemeColor`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BackgroundColor:Option<Value>,

	/// If `true`, the item is aligned to the left of the status bar.
	pub IsAlignedLeft:bool,

	/// The priority of this item. Higher numbers are shown more to the left
	/// (for left-aligned items) or more to the right (for right-aligned
	/// items).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Priority:Option<f64>,

	/// Accessibility information for screen readers. Serialized
	/// `AccessibilityInformation`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AccessibilityInformation:Option<Value>,
}
