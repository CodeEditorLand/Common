use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct InputBoxOptionsDto {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Title:Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub PlaceHolder:Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Value:Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Prompt:Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Password:Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IgnoreFocusOut:Option<bool>,
}
