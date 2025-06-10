/// @module StatusBarEntryDto
/// @description Defines the Data Transfer Object for a single status bar item.
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct that represents the complete state of a
/// `vscode.StatusBarItem`.
///
/// This DTO is sent from Cocoon to Mountain whenever an extension creates or
/// updates a status bar item, providing the host with all the information
/// needed to render it.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StatusBarEntryDto {
	/// An internal, host-generated unique ID for this entry instance.
	pub EntryId:String,

	/// The identifier of the status bar item, as provided by the extension.
	pub ItemId:String,

	/// The identifier of the extension that owns this status bar item.
	pub ExtensionId:String,

	/// An optional name for the status bar item, used for identification.
	pub Name:String,

	/// The text to be displayed for this item (can include icons like
	/// `$(icon)`).
	pub Text:String,

	/// The tooltip to show when hovering over the item.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Tooltip:Option<Value>, // DTO: string | IMarkdownStringDto

	/// A flag indicating if the extension host has a dynamic tooltip provider
	/// for this item.
	pub HasTooltipProvider:bool,

	/// The command to execute when the item is clicked.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Command:Option<Value>, // DTO: CommandDto

	/// The foreground color for this item.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Color:Option<Value>, // DTO: string | ThemeColor

	/// The background color for this item.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BackgroundColor:Option<Value>, // DTO: ThemeColor

	/// If `true`, the item is aligned to the left of the status bar.
	pub AlignLeft:bool,

	/// The priority of this item. Higher numbers are shown more to the left
	/// (for left-aligned items) or more to the right (for right-aligned
	/// items).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Priority:Option<f64>,

	/// Accessibility information for screen readers.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AccessibilityInformation:Option<Value>, // DTO: AccessibilityInformation
}
