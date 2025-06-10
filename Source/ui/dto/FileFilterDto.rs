use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct FileFilterDto {
	pub Name:String,
	pub ExtensionList:Vec<String>,
}
