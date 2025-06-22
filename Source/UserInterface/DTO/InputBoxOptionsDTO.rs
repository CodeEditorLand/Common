//! # InputBoxOptionsDTO
//!
//! Defines the Data Transfer Object for the options of a user input box.

use serde::{Deserialize, Serialize};

/// A serializable struct that holds all configuration options for a user input
/// box.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct InputBoxOptionsDTO {
	/// An optional title for the input box window.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Title:Option<String>,

	/// Placeholder text to show in the input field when it is empty.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PlaceHolder:Option<String>,

	/// An initial value to populate in the input field.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Value:Option<String>,

	/// A descriptive prompt message shown to the user.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Prompt:Option<String>,

	/// If `true`, the input will be masked (e.g., for passwords).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IsPassword:Option<bool>,

	/// If `true`, the input box will not close when it loses focus.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IgnoreFocusOut:Option<bool>,
}
