use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct RangeDto {
	pub StartLineNumber:u32,
	pub StartColumn:u32,
	pub EndLineNumber:u32,
	pub EndColumn:u32,
}
