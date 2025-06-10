use serde::{Deserialize, Serialize};

use super::{IMarkdownStringDto::IMarkdownStringDto, RangeDto::RangeDto};
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct HoverResultDto {
	pub Contents:Vec<IMarkdownStringDto>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Range:Option<RangeDto>,
}
