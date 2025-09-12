//! # OpenDialogOptionsDTO
//!
//! Defines the Data Transfer Object for the options of a file open dialog.

use serde::{Deserialize, Serialize};

use super::DialogOptionsDTO::DialogOptionsDTO;

/// A serializable struct that holds all configuration options for an "Open"
/// file dialog. It flattens the shared `DialogOptionsDTO` and adds properties
/// specific to opening files or folders.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OpenDialogOptionsDTO {
	/// The base options common to all file dialogs.
	#[serde(flatten)]
	pub Base:DialogOptionsDTO,

	/// If `true`, the user can select multiple files or folders.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CanSelectMany:Option<bool>,

	/// If `true`, the dialog allows the user to select folders instead of
	/// files.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CanSelectFolders:Option<bool>,

	/// If `true`, the dialog allows the user to select files.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CanSelectFiles:Option<bool>,
}
