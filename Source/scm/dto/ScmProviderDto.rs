/// @module ScmProviderDto
/// @description Defines the DTO for an SCM provider itself.
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct representing the metadata for a source control
/// provider.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ScmProviderDto {
	pub Handle:u32,
	pub Label:String,
	pub RootUri:Option<Value>, // DTO: UriComponents
	pub Count:Option<u32>,
	pub CommitTemplate:Option<String>,
}
