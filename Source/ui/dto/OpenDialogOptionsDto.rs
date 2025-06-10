use serde::{Deserialize, Serialize};

use super::DialogOptionsDto::DialogOptionsDto;
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OpenDialogOptionsDto {
	#[serde(flatten)]
	pub Base:DialogOptionsDto,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Multiple:Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Directory:Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Recursive:Option<bool>,
}
