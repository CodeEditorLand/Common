//! # SaveDialogOptionsDTO
//!
//! Defines the Data Transfer Object for the options of a file save dialog.

use serde::{Deserialize, Serialize};

use super::DialogOptionsDTO::DialogOptionsDTO;

/// A serializable struct that holds all configuration options for a "Save"
/// file dialog. It flattens the shared `DialogOptionsDTO` and can be extended
/// with save-specific properties in the future.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SaveDialogOptionsDTO {
	/// The base options common to all file dialogs.
	#[serde(flatten)]
	pub Base:DialogOptionsDTO,
}
