use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MessageOptionsDto {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Title:Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Modal:Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Detail:Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ItemList:Option<Vec<String>>,
}
