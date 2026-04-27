//! # SourceControlCreateDTO
//!
//! Defines the DTO for creating a new Source Control provider.

use serde::{Deserialize, Serialize};
use url::Url;

use crate::Utility::Serialization::URLSerializationHelper;

/// A serializable struct sent from the extension host (`Cocoon`) to the main
/// host (`Mountain`) when an extension calls `vscode.scm.createSourceControl`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceControlCreateDTO {
	/// The unique identifier for this source control provider.
	#[serde(rename = "id")]
	pub ID:String,

	/// The human-readable label for this provider (e.g., "Git").
	pub Label:String,

	/// The root URI of the repository this provider is managing.
	#[serde(with = "URLSerializationHelper")]
	pub RootUri:Url,

	/// Caller-supplied provider handle. Cocoon uses a process-local
	/// sequential allocator (`NextProviderHandle()` in `ScmNamespace.ts`)
	/// and includes the same handle on every subsequent
	/// `register_scm_resource_group` / `update_scm_group` /
	/// `unregister_scm_provider` notification. When present, Mountain MUST
	/// key its marker maps under this handle so group-update lookups
	/// resolve - allocating a fresh handle here keys the
	/// `SourceControlManagementProviders` map under a value Cocoon will
	/// never reference, and every later `update_scm_group` warns
	/// "Received group update for unknown provider handle: <H>" while the
	/// SCM viewlet stays empty. Optional so callers without an external
	/// handle (none currently) still work via Mountain-side allocation.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub Handle:Option<u32>,
}
