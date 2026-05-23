//! # SourceControlGroupUpdateDTO
//!
//! Defines the DTO for updating the properties of an SourceControlManagement
//! resource group.

use serde::{Deserialize, Serialize};

/// A serializable struct used to update the properties of a source control
/// group, such as its label or visibility, without affecting its list of
/// resources.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceControlGroupUpdateDTO {
	/// The handle of the provider that owns the group. Optional: callers that
	/// pass the handle as a positional argument may omit this field.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ProviderHandle:Option<u32>,

	/// The unique identifier for the group within its provider.
	#[serde(rename = "groupId")]
	pub GroupID:String,

	/// The new human-readable label for the group.
	pub Label:String,
	// Other properties like `HideWhenEmpty` could be added here.
}
