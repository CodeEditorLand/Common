use serde::{Deserialize, Serialize};

use super::FileFilterDto::FileFilterDto;
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DialogOptionsDto {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Title:Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub DefaultPath:Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FilterList:Option<Vec<FileFilterDto>>,
}
