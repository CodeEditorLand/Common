/// @module ConfigurationOverridesDto
/// @description Defines the Data Transfer Object for specifying configuration
/// overrides.
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct for specifying overrides when retrieving or inspecting
/// configuration values. This allows for fetching settings that are specific to
/// a particular resource (like a file) or a language.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigurationOverridesDto {
	/// The resource URI to which the configuration should be scoped.
	/// This is used to resolve resource-specific and folder-specific settings.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Resource:Option<Value>, // DTO: UriComponents

	/// The language identifier for language-specific overrides.
	/// This is used to resolve `[language]` blocks in `settings.json`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub OverrideIdentifier:Option<String>,
}
