/// @module ScmGroupDto
/// @description Defines the DTO for an SCM resource group (e.g., "Changes",
/// "Staged Changes").
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct representing a group of source control resources.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ScmGroupDto {
	pub ProviderHandle:u32,
	pub Id:String,
	pub Label:String,
	// Add other properties like `HideWhenEmpty`, etc.
}
