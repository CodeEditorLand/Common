//! # InspectResultDataDTO
//!
//! Defines the Data Transfer Object for the result of inspecting a
//! configuration key across all possible scopes.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct that represents the detailed breakdown of a
/// configuration value from all possible sources.
///
/// This DTO is returned by the `InspectConfiguration` effect and is used by
/// UI components like the Settings editor to show where values are inherited
/// from, what the default is, and what the final effective value is.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct InspectResultDataDTO {
	/// The value from the default configuration.
	pub DefaultValue:Option<Value>,

	/// The value from the user (global) settings.
	pub UserValue:Option<Value>,

	/// The value from the local (un-synced) user settings.
	pub UserLocalValue:Option<Value>,

	/// The value from the remote user settings (if applicable).
	pub UserRemoteValue:Option<Value>,

	/// The value from the workspace settings.
	pub WorkSpaceValue:Option<Value>,

	/// The value from a specific workspace folder's settings.
	pub WorkSpaceFolderValue:Option<Value>,

	/// The value from the in-memory configuration.
	pub MemoryValue:Option<Value>,

	/// The value from the policy-enforced configuration.
	pub PolicyValue:Option<Value>,

	/// The final, effective value after all scopes have been merged.
	pub EffectiveValue:Option<Value>,

	// --- Language-Specific Overrides ---
	pub DefaultLanguageValue:Option<Value>,

	pub UserLanguageValue:Option<Value>,

	pub UserLocalLanguageValue:Option<Value>,

	pub UserRemoteLanguageValue:Option<Value>,

	pub WorkSpaceLanguageValue:Option<Value>,

	pub WorkSpaceFolderLanguageValue:Option<Value>,

	pub MemoryLanguageValue:Option<Value>,

	pub PolicyLanguageValue:Option<Value>,

	/// A list of language identifiers for which language-specific values were
	/// found during the inspection.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LanguageIdentifiers:Option<Vec<String>>,
}
