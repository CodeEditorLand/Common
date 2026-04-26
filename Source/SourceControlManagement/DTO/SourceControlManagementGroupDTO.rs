//! # SourceControlManagementGroupDTO
//!
//! Defines the DTO for an SourceControlManagement resource group (e.g.,
//! "Changes", "Staged Changes").

use serde::{Deserialize, Serialize};

/// A serializable struct representing a group of source control resources.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceControlManagementGroupDTO {
	pub ProviderHandle:u32,

	pub Identifier:String,

	pub Label:String,
	// Add other properties like `HideWhenEmpty`, etc.
}
