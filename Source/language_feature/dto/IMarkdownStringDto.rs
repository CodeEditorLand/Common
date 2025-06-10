use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IMarkdownStringDto {
	pub Value:String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub IsTrusted:Option<Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SupportThemeIcons:Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SupportHtml:Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub BaseUri:Option<Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Uris:Option<HashMap<String, Value>>,
}
