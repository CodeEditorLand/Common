//! # HoverResultDTO
//!
//! Defines the Data Transfer Object for the result of a hover provider request.

use serde::{Deserialize, Serialize};

use super::{IMarkdownStringDTO::IMarkdownStringDTO, RangeDTO::RangeDTO};

/// A serializable struct that represents the content to be displayed in a
/// hover tooltip, analogous to `vscode.Hover`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct HoverResultDTO {
	/// The contents of the hover, which can be one or more markdown strings.
	pub Contents:Vec<IMarkdownStringDTO>,
	/// An optional range to which this hover applies. When not specified, the
	/// range of the word at the request position is used.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Range:Option<RangeDTO>,
}
