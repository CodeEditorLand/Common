use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct PositionDto {
	pub LineNumber:u32,
	pub Column:u32,
}
