//! # SourceControlUpdateDTO
//!
//! Defines a generic DTO for updating properties of an SourceControlManagement provider.

use serde::{Deserialize, Serialize};

/// A serializable struct used to send updates for a source control provider's
/// top-level properties, such as the commit message in the input box or the
/// badge count.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SourceControlUpdateDTO {
	/// The handle of the provider to update.
	pub ProviderHandle:u32,
	/// The new value for the commit message input box.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InputBoxValue:Option<String>,
	/// The new count to display as a badge on the SourceControlManagement icon.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Count:Option<u32>,
	// This could be expanded to include other updatable properties.
}
