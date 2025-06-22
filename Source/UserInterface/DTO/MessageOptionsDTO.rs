//! # MessageOptionsDTO
//!
//! Defines the Data Transfer Object for the options of a user-facing message
//! dialog.

use serde::{Deserialize, Serialize};

/// A serializable struct that holds all configuration options for a message
/// shown to the user via `ShowMessage`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MessageOptionsDTO {
	/// An optional title for the message dialog window.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Title:Option<String>,

	/// If `true`, the message will be modal, blocking interaction with the
	/// rest of the UI until it is dismissed.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IsModal:Option<bool>,

	/// Optional, more detailed text to display in the message body.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Detail:Option<String>,

	/// A list of string titles for action buttons to display on the message.
	/// The selected button's title is returned by the `ShowMessage` effect.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ItemList:Option<Vec<String>>,
}
