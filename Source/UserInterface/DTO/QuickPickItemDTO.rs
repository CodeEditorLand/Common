//! # QuickPickItemDTO
//!
//! Defines the Data Transfer Object for a single item in a quick pick list.

use serde::{Deserialize, Serialize};

/// A serializable struct that represents a single selectable item within a
/// quick pick UI.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct QuickPickItemDTO {
	/// The primary text label for the item.
	pub Label:String,
	/// An optional description shown in a separate column to the right of the
	/// label.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Description:Option<String>,
	/// An optional detail text shown underneath the label.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Detail:Option<String>,
	/// If `true`, this item will be initially selected.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Picked:Option<bool>,
	/// If `true`, this item will always be shown at the top of the list, even
	/// when filtering.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AlwaysShow:Option<bool>,
}
