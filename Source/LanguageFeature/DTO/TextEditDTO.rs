//! # TextEditDTO
//!
//! Defines the Data Transfer Object for representing a single text edit
//! operation.

use serde::{Deserialize, Serialize};

use super::RangeDTO::RangeDTO;

/// A serializable struct representing a text edit, analogous to
/// `vscode.TextEdit`. It is the fundamental building block for formatting edits
/// and workspace edits.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextEditDTO {
	/// The range of text to be replaced.
	pub Range:RangeDTO,

	/// The new text to be inserted.
	pub NewText:String,
}
