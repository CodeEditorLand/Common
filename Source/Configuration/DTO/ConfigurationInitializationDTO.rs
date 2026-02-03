//! # ConfigurationInitializationDTO
//!
//! Defines the Data Transfer Object for the complete, multi-layered
//! configuration state of the application.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct that represents the initial configuration data
/// structure.
///
/// This DTO is typically sent from the main application (`Mountain`) to a
/// sidecar (`Cocoon`) on startup. It provides the sidecar with a complete
/// snapshot of all configuration sources, allowing it to accurately reflect the
/// application's settings state without needing to read files itself.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigurationInitializationDTO {
	/// The final, merged configuration values after applying all scopes in the
	/// correct order of precedence. This is the "effective" configuration.
	pub Effective:Value,

	/// The default values for all configurations, as defined by the application
	/// and its extensions.
	pub Defaults:Value,

	/// User-level settings, typically from a global `settings.json` file.
	pub User:Value,

	/// Workspace-level settings, from a `.code-workspace` file or a folder's
	/// `.vscode/settings.json`.
	pub Workspace:Value,

	/// Settings specific to individual folders in a multi-root workspace.
	pub Folders:Value,

	/// Temporary, in-memory settings that are not persisted.
	pub Memory:Value,

	/// Policy-enforced settings that cannot be overridden by the user.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Policy:Option<Value>,

	/// Detailed scope information for each configuration key.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ConfigurationScopes:Option<Vec<(String, Value)>>,
}
