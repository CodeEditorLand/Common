//! # RangeDTO
//!
//! Defines the Data Transfer Object for representing a range of text in a
//! document, spanning from a start position to an end position.

use serde::{Deserialize, Serialize};

/// A serializable struct representing a range in a text document. A range is
/// defined by its start and end positions and is a fundamental building block
/// for operations like selections, highlights, and text edits.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct RangeDTO {
	/// The starting line number of the range (zero-based).
	pub StartLineNumber:u32,
	/// The starting column of the range (zero-based).
	pub StartColumn:u32,
	/// The ending line number of the range (zero-based).
	pub EndLineNumber:u32,
	/// The ending column of the range (zero-based).
	pub EndColumn:u32,
}
