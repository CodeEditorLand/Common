//! # SourceControlManagementProviderDTO
//!
//! Defines the DTO for an SCM provider itself.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct representing the metadata for a source control
/// provider.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SourceControlManagementProviderDTO {
	pub Handle:u32,
	pub Label:String,
	/// The root URI of the repository this provider is managing. Serialized
	/// `UriComponents`.
	pub RootURI:Option<Value>,
	/// An optional count of changed resources, often displayed as a badge.
	pub Count:Option<u32>,
	/// The template for the commit message input box.
	pub CommitTemplate:Option<String>,
}
