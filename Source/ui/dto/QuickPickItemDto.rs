use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct QuickPickItemDto {
	pub Label:String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Description:Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Detail:Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Picked:Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AlwaysShow:Option<bool>,
}
