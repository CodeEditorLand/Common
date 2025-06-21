//! # CompletionListDTO
//!
//! Defines the Data Transfer Object for a list of completion items.

use serde::{Deserialize, Serialize};

use super::CompletionItemDTO::CompletionItemDTO;

/// A serializable struct representing a list of completion items, analogous to
/// `vscode.CompletionList`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CompletionListDTO {
	/// The completion items.
	pub Suggestions:Vec<CompletionItemDTO>,

	/// A flag indicating if this completion list is incomplete.
	#[serde(default, skip_serializing_if = "std::ops::Not::not")]
	pub IsIncomplete:bool,
}
