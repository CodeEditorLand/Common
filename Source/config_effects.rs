// Land_Common/src/config_effects.rs
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::environment::{Environment, Requires}; // For trait bounds and effect context
use crate::runtime::AppRuntime; // Assumed concrete runtime accessor for effects
use crate::{effect::ActionEffect, errors::CommonError};

// --- Configuration Enums and DTOs ---

/// Defines the target level for a configuration update.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")] // For consistency if serialized, though u32 doesn't need it
pub enum ConfigurationTarget {
	UserLocal = 1,       // User settings specific to the current machine/installation.
	User = 2,            // User settings, potentially synced across machines.
	Workspace = 3,       // Settings applicable to the entire workspace.
	WorkspaceFolder = 4, // Settings specific to a folder within a multi-root workspace.
	Default = 5,         // Default values provided by the application or extensions.
	Memory = 6,          // In-memory overrides, not persisted.
	Policy = 7,          // Configuration enforced by policies (e.g., enterprise).
}

/// Defines the scope of a configuration value, indicating where it can be
/// applied. Aligns with VS Code's `vscode.ConfigurationScope`.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ConfigurationScope {
	Application = 1,        // Applies to the entire application, not specific to any window/workspace.
	Machine = 2,            /* Applies to the machine, not specific to user or workspace. (Less common for user
	                         * settings) */
	Window = 3,             // Applies to a specific window/workspace instance.
	Resource = 4,           // Applies to a specific resource (e.g., file URI).
	LanguageDefined = 5,    // Language-specific override for a setting at any of the above scopes.
	MachineOverridable = 6, // Like Machine, but can be overridden by workspace/user settings.
}

/// DTO for specifying overrides when retrieving or inspecting configuration
/// values. This often includes a resource URI (for resource or folder-specific
/// settings) and a language identifier (for language-specific settings).
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct IConfigurationOverrides {
	/// The resource URI to which the configuration should be scoped.
	/// Typically a `Value` representing `UriComponents` DTO.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub resource:Option<Value>,

	/// The language identifier for language-specific overrides.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub override_identifier:Option<String>,
	// VS Code also has `overrideIdentifiers` (plural, string[]) for more complex language overrides.
	// This DTO uses a single `override_identifier` for simplicity.
}

/// DTO representing the initial configuration data structure, often sent
/// from the main thread (Mountain) to a sidecar (Cocoon) on startup or after
/// significant configuration changes.
/// It includes values from different configuration sources (default, user,
/// workspace, etc.).
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IConfigurationInitDataDto {
	pub effective:Value, // The final, merged configuration values.
	pub defaults:Value,  // Default values (e.g., `{ "contents": { "editor.fontSize": 12 } }`).
	pub user:Value,      // User-level settings.
	pub workspace:Value, // Workspace-level settings.
	// Array of `[UriComponentsDto, { "contents": object }]` for multi-root workspace folder settings.
	pub folders:Value,
	pub memory:Value, // In-memory settings.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub policy:Option<Value>, // Policy-enforced settings.
	// Optional: Detailed scope information for each configuration key.
	// Array of `[key_string, ConfigurationScope_enum_as_Value]`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub configuration_scopes:Option<Vec<(String, Value)>>,
}

/// DTO for the result of inspecting a configuration key.
/// Provides values from different scopes (user, workspace, default, etc.)
/// and the final effective value.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectResultData {
	// Using Value for flexibility, as the actual type `T` of a config key can vary.
	pub default_value:Option<Value>,
	pub user_value:Option<Value>,        // Combined local & remote user settings.
	pub user_local_value:Option<Value>,  // Specifically local user settings.
	pub user_remote_value:Option<Value>, // Specifically remote/synced user settings.
	pub workspace_value:Option<Value>,
	pub workspace_folder_value:Option<Value>,
	pub memory_value:Option<Value>,    // In-memory overrides.
	pub policy_value:Option<Value>,    // Policy-enforced values.
	pub effective_value:Option<Value>, // The final value after all merging and overrides.

	// Values specific to a language override, if applicable.
	pub default_language_value:Option<Value>,
	pub user_language_value:Option<Value>,
	pub user_local_language_value:Option<Value>,
	pub user_remote_language_value:Option<Value>,
	pub workspace_language_value:Option<Value>,
	pub workspace_folder_language_value:Option<Value>,
	pub memory_language_value:Option<Value>,
	pub policy_language_value:Option<Value>,

	/// List of language identifiers for which language-specific overrides exist
	/// for this key.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language_ids:Option<Vec<String>>,
	// More detailed scope information (similar to VS Code's IConfigurationInspectInfo<T>)
	// could be added here if needed, e.g., indicating which scope provided the override.
	// pub default: Option<{value?: Value, override?: Value, scope?: ConfigurationScope}>,
	// pub user: Option<{value?: Value, override?: Value, scope?: ConfigurationScope}>,
	// ... etc.
}

// --- Configuration Provider Traits ---

/// Trait for an environment component that can provide and update configuration
/// values.
#[async_trait]
pub trait ConfigProvider: Environment {
	/// Retrieves a configuration value for a given section/key, applying
	/// specified overrides.
	async fn get_configuration_value(
		&self,
		section:Option<String>, // Dot-separated key, or None for all.
		overrides:IConfigurationOverrides,
	) -> Result<Value, CommonError>;

	/// Updates a configuration value at a specific key and target scope.
	async fn update_configuration_value(
		&self,
		key:String,                        // Dot-separated key to update.
		value:Value,                       // New value (Value::Null to remove).
		target:ConfigurationTarget,        // Where to write the update (User, Workspace, etc.).
		overrides:IConfigurationOverrides, // Context for WorkspaceFolder target or language overrides.
		scope_to_language:Option<bool>,    // If true, update within language-specific section `[languageId]`.
	) -> Result<(), CommonError>;
}

/// Trait for an environment component that can inspect configuration values,
/// providing details about their sources and effective values.
#[async_trait]
pub trait ConfigInspector: Environment {
	/// Inspects a configuration key to get its value from all relevant scopes.
	async fn inspect_configuration_value(
		&self,
		key:String,                        // Dot-separated key to inspect.
		overrides:IConfigurationOverrides, // Context for resource/language.
	) -> Result<Option<InspectResultData>, CommonError>;
}

// --- Effect Constructors ---

/// Creates an effect to get a configuration value.
pub fn get_configuration(
	section:Option<String>,
	overrides_dto_val:Value,         // IConfigurationOverrides serialized as JSON Value
	_scope_to_language:Option<bool>, /* This param seems misplaced for get_configuration, more for update. Included
	                                  * if protocol sends it. */
) -> ActionEffect<Arc<AppRuntime>, CommonError, Value> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let section_clone = section.clone();
		let overrides_val_clone = overrides_dto_val.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn ConfigProvider + Send + Sync> = concrete_env.require();

			let overrides_parsed:IConfigurationOverrides = serde_json::from_value(overrides_val_clone.clone())
				.map_err(|e| {
					CommonError::InvalidArg(
						"overrides_dto".to_string(),
						format!("Failed to parse IConfigurationOverrides from {:?}: {}", overrides_val_clone, e),
					)
				})?;
			provider.get_configuration_value(section_clone, overrides_parsed).await
		})
	}))
}

/// Creates an effect to update a configuration value.
pub fn update_configuration(
	key:String,
	value:Value,
	target_as_u32:u32,       // ConfigurationTarget enum value as u32
	overrides_dto_val:Value, // IConfigurationOverrides serialized as JSON Value
	scope_to_language:Option<bool>,
) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let key_clone = key.clone();
		let value_clone = value.clone();
		let overrides_val_clone = overrides_dto_val.clone();
		let stl_clone = scope_to_language;

		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn ConfigProvider + Send + Sync> = concrete_env.require();

			// Deserialize ConfigurationTarget from u32 via Value
			let target_parsed:ConfigurationTarget =
				serde_json::from_value(Value::from(target_as_u32)).map_err(|e| {
					CommonError::InvalidArg(
						"target".to_string(),
						format!("Failed to parse ConfigurationTarget from u32 {}: {}", target_as_u32, e),
					)
				})?;

			let overrides_parsed:IConfigurationOverrides = serde_json::from_value(overrides_val_clone.clone())
				.map_err(|e| {
					CommonError::InvalidArg(
						"overrides_dto".to_string(),
						format!("Failed to parse IConfigurationOverrides from {:?}: {}", overrides_val_clone, e),
					)
				})?;

			provider
				.update_configuration_value(key_clone, value_clone, target_parsed, overrides_parsed, stl_clone)
				.await
		})
	}))
}

/// Creates an effect to inspect a configuration value.
pub fn inspect_configuration_value(
	key:String,
	overrides_dto_val:Value, // IConfigurationOverrides serialized as JSON Value
) -> ActionEffect<Arc<AppRuntime>, CommonError, Option<InspectResultData>> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let key_clone = key.clone();
		let overrides_val_clone = overrides_dto_val.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let inspector:Arc<dyn ConfigInspector + Send + Sync> = concrete_env.require();

			let overrides_parsed:IConfigurationOverrides = serde_json::from_value(overrides_val_clone.clone())
				.map_err(|e| {
					CommonError::InvalidArg(
						"overrides_dto".to_string(),
						format!("Failed to parse IConfigurationOverrides from {:?}: {}", overrides_val_clone, e),
					)
				})?;
			inspector.inspect_configuration_value(key_clone, overrides_parsed).await
		})
	}))
}
