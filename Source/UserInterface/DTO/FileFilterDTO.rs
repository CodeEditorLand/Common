//! # FileFilterDTO
//!
//! Defines the Data Transfer Object for a single file filter in a native file
//! dialog.

use serde::{Deserialize, Serialize};

/// A serializable struct that represents a single selectable filter in a file
/// dialog's "files of type" dropdown.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileFilterDTO {
	/// The human-readable name of the filter (e.g., "Image Files").
	pub Name:String,

	/// A list of file extensions associated with this filter (e.g., `["jpg",

	/// "png"]`).
	pub ExtensionList:Vec<String>,
}
