/// @module ScmResourceDto
/// @description Defines the DTO for a single resource under source control
/// (e.g., a changed file).
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct representing a single item within an SCM group.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ScmResourceDto {
	pub ProviderHandle:u32,
	pub GroupId:String,
	pub ResourceUri:Value, // DTO: UriComponents
	pub Decorations:Value, // DTO for decorations like color, tooltip, etc.
}
