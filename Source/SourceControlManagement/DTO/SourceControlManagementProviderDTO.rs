//! # SourceControlManagementProviderDTO
//!
//! Defines the DTO for an SourceControlManagement provider itself.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::SourceControlManagement::DTO::SourceControlInputBoxDTO::SourceControlInputBoxDTO;

/// A serializable struct representing the metadata for a source control
/// provider.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceControlManagementProviderDTO {
	pub Handle:u32,

	/// The provider id ("git", "github", "hg", …). Used as the
	/// `scmId` field on the `sky://scm/register` event payload so
	/// replay-on-late-listener (Sky boot race) can re-emit with
	/// the original id Cocoon's `ScmShimRegistry` keys against.
	/// Defaults to empty for backwards-compat with old DTOs.
	#[serde(default)]
	pub Identifier:String,

	pub Label:String,

	/// The root URI of the repository this provider is managing. Serialized
	/// `UriComponents`.
	#[serde(rename = "rootUri")]
	pub RootURI:Option<Value>,

	/// An optional count of changed resources, often displayed as a badge.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Count:Option<u32>,

	/// The template for the commit message input box.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CommitTemplate:Option<String>,

	/// The command executed when the user accepts the SCM input box (e.g.
	/// Ctrl+Enter).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AcceptInputCommand:Option<Value>,

	/// The state of the SourceControlManagement input box (commit message
	/// area).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InputBox:Option<SourceControlInputBoxDTO>,
}
