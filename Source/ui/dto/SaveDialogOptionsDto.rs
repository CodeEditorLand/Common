use serde::{Deserialize, Serialize};

use super::DialogOptionsDto::DialogOptionsDto;
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SaveDialogOptionsDto {
	#[serde(flatten)]
	pub Base:DialogOptionsDto,
}
