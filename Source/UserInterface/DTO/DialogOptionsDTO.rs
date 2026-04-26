//! # DialogOptionsDTO
//!
//! Defines the Data Transfer Object for shared options across different types
//! of file dialogs.

use serde::{Deserialize, Serialize};

use super::FileFilterDTO::FileFilterDTO;

/// A serializable struct that holds common configuration options for native
/// file dialogs, such as `ShowOpenDialog` and `ShowSaveDialog`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DialogOptionsDTO {
	/// The title of the dialog window.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Title:Option<String>,

	/// The default path that the dialog should open to.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DefaultPath:Option<String>,

	/// A list of file filters that the user can select from.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FilterList:Option<Vec<FileFilterDTO>>,
}
