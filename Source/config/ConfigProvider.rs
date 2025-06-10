use async_trait::async_trait;
use serde_json::Value;

/// @module ConfigProvider
/// @description Defines the abstract service trait for providing and updating
/// configuration values.
use super::dto::{ConfigurationOverridesDto, ConfigurationTarget};
use crate::{environment::Environment, error::CommonError};

/// An abstract service contract for an environment component that can provide
/// the final, merged configuration values and handle requests to update those
/// values in their persistent storage (i.e., `settings.json` files).
#[async_trait]
pub trait ConfigProvider: Environment + Send + Sync {
	/// Retrieves a configuration value for a given section/key, applying
	/// specified overrides. This should return the final, effective value
	/// after merging all configuration sources.
	///
	/// @param Section - An optional, dot-separated key to a specific
	/// configuration section.   If `None`, the entire configuration object
	/// should be returned. @param Overrides - A DTO specifying scope overrides
	/// (e.g., for a specific resource).
	///
	/// @returns A `Result` containing the requested configuration as a
	/// `serde_json::Value`.
	async fn GetConfigurationValue(
		&self,
		Section:Option<String>,
		Overrides:ConfigurationOverridesDto,
	) -> Result<Value, CommonError>;

	/// Updates a configuration value at a specific key and target scope.
	///
	/// @param Key - The dot-separated configuration key to update.
	/// @param ValueToSet - The new `serde_json::Value` to set for the key.
	/// @param Target - The `ConfigurationTarget` enum specifying which scope to
	/// write to   (e.g., User, Workspace).
	/// @param Overrides - A DTO for scope overrides.
	/// @param ScopeToLanguage - An optional flag related to language-specific
	/// settings.
	async fn UpdateConfigurationValue(
		&self,
		Key:String,
		ValueToSet:Value,
		Target:ConfigurationTarget,
		Overrides:ConfigurationOverridesDto,
		ScopeToLanguage:Option<bool>,
	) -> Result<(), CommonError>;
}
