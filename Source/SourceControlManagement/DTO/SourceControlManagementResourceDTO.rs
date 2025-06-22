//! # SourceControlManagementResourceDTO
//!
//! Defines the DTO for a single resource under source control (e.g., a changed
//! file).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct representing a single item within an SourceControlManagement group, such
/// as a file that has been modified, added, or deleted.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SourceControlManagementResourceDTO {
	pub ProviderHandle:u32,
	pub GroupIdentifier:String,
	/// The URI of the resource. Serialized `UriComponents`.
	pub ResourceURI:Value,
	/// A DTO for decorations like color, tooltip, and file status icons.
	/// Serialized `SourceControlResourceDecorations`.
	pub Decorations:Value,
}
