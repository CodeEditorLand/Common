//! # QuickPickOptionsDTO
//!
//! Defines the Data Transfer Object for the options of a quick pick UI.

use serde::{Deserialize, Serialize};

/// A serializable struct that holds all configuration options for a quick pick
/// UI.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct QuickPickOptionsDTO {
	/// An optional title for the quick pick window.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Title:Option<String>,
	/// Placeholder text to show in the filter input box when it is empty.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PlaceHolder:Option<String>,
	/// If `true`, the user can select more than one item.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CanPickMany:Option<bool>,
	/// If `true`, the quick pick will not close when it loses focus.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IgnoreFocusOut:Option<bool>,
}
