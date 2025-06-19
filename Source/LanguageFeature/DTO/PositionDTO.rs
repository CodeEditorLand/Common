//! # PositionDTO
//!
//! Defines the Data Transfer Object for representing a zero-based line and
//! column position in a text document.

use serde::{Deserialize, Serialize};

/// A serializable struct representing a position in a text document,
/// consisting of a line number and a column number. This is a fundamental
/// building block for many other DTOs.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct PositionDTO {
	/// The zero-based line number.
	pub LineNumber:u32,
	/// The zero-based column number.
	pub Column:u32,
}
