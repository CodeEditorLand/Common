//! # SourceControlInputBoxDTO
//!
//! Defines the DTO for the SourceControlManagement input box (commit message
//! box).

use serde::{Deserialize, Serialize};

/// A serializable struct representing the state of the commit message input box
/// associated with a source control provider.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SourceControlInputBoxDTO {
	/// The current text content of the input box.
	pub Value:String,

	/// Placeholder text to show when the input box is empty.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Placeholder:Option<String>,

	/// Whether the input box is currently visible.
	#[serde(default)]
	pub Visible:bool,
}
