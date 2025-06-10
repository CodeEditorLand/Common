/// @module InspectResultDataDto
/// @description Defines the Data Transfer Object for the result of inspecting
/// a configuration key across all scopes.
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct that represents the detailed breakdown of a
/// configuration value from all possible sources.
///
/// This DTO is returned by the `InspectConfiguration` effect and is used by UI
/// like the Settings editor to show where values are inherited from.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct InspectResultDataDto {
	/// The value from the default configuration.
	pub DefaultValue:Option<Value>,
	/// The value from the user (global) settings.
	pub UserValue:Option<Value>,
	/// The value from the local (un-synced) user settings.
	pub UserLocalValue:Option<Value>,
	/// The value from the remote user settings.
	pub UserRemoteValue:Option<Value>,
	/// The value from the workspace settings.
	pub WorkspaceValue:Option<Value>,
	/// The value from the workspace folder settings.
	pub WorkspaceFolderValue:Option<Value>,
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
	pub WorkspaceLanguageValue:Option<Value>,
	pub WorkspaceFolderLanguageValue:Option<Value>,
	pub MemoryLanguageValue:Option<Value>,
	pub PolicyLanguageValue:Option<Value>,

	/// A list of language identifiers for which language-specific values were
	/// found.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub LanguageIdentifiers:Option<Vec<String>>,
}
