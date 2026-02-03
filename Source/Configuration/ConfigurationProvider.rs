// File: Common/Source/Configuration/ConfigurationProvider.rs
// Role: Defines the abstract service trait for configuration management.
// Responsibilities:
//   - Provide a contract for retrieving merged configuration values.
//   - Provide a contract for updating configuration values at specific targets
//     (e.g., User, Workspace).

use async_trait::async_trait;
use serde_json::Value;

use super::DTO::{ConfigurationOverridesDTO::ConfigurationOverridesDTO, ConfigurationTarget::ConfigurationTarget};
use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can provide
/// the final, merged configuration values and handle requests to update those
/// values in their persistent storage (e.g., `settings.json` files).
#[async_trait]
pub trait ConfigurationProvider: Environment + Send + Sync {
	/// Retrieves a configuration value for a given section or key, applying
	/// specified overrides.
	///
	/// This method should return the final, effective value after merging all
	/// configuration sources (e.g., default, user, workspace) in the correct
	/// order of precedence.
	///
	/// # Parameters
	///
	/// * `Section`: An optional, dot-separated key to a specific configuration
	///   section. If `None`, the entire configuration object should be
	///   returned.
	/// * `Overrides`: A DTO specifying scope overrides (e.g., for a specific
	///   resource or language).
	///
	/// # Returns
	///
	/// A `Result` containing the requested configuration as a
	/// `serde_json::Value`.
	async fn GetConfigurationValue(
		&self,

		Section:Option<String>,

		Overrides:ConfigurationOverridesDTO,
	) -> Result<Value, CommonError>;

	/// Updates a configuration value at a specific key and target scope.
	///
	/// # Parameters
	///
	/// * `Key`: The dot-separated configuration key to update.
	/// * `ValueToSet`: The new `serde_json::Value` to set for the key.
	/// * `Target`: The `ConfigurationTarget` enum specifying which scope to
	///   write to (e.g., User, Workspace).
	/// * `Overrides`: A DTO for scope overrides.
	/// * `ScopeToLanguage`: An optional flag related to language-specific
	///   settings.
	async fn UpdateConfigurationValue(
		&self,

		Key:String,

		ValueToSet:Value,

		Target:ConfigurationTarget,

		Overrides:ConfigurationOverridesDTO,

		ScopeToLanguage:Option<bool>,
	) -> Result<(), CommonError>;
}
