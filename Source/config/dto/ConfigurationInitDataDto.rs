/// @module ConfigurationInitDataDto
/// @description Defines the Data Transfer Object for the complete,
/// multi-layered configuration state of the application.
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct that represents the initial configuration data
/// structure.
///
/// This DTO is typically sent from the main thread (`Mountain`) to a sidecar
/// (`Cocoon`) on startup, providing the sidecar with a complete snapshot of all
/// configuration sources.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigurationInitDataDto {
	/// The final, merged configuration values after applying all scopes in the
	/// correct order.
	pub Effective:Value,

	/// The default values for all configurations.
	pub Defaults:Value,

	/// User-level settings from the global `settings.json`.
	pub User:Value,

	/// Workspace-level settings from the `.code-workspace` file or folder
	/// settings.
	pub Workspace:Value,

	/// Settings specific to individual folders in a multi-root workspace.
	pub Folders:Value,

	/// Temporary, in-memory settings.
	pub Memory:Value,

	/// Policy-enforced settings that cannot be overridden by the user.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Policy:Option<Value>,

	/// Detailed scope information for each configuration key.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConfigurationScopes:Option<Vec<(String, Value)>>,
}
