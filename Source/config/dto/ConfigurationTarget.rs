/// @module ConfigurationTarget
/// @description Defines the Data Transfer Object enum for specifying the target
/// scope of a configuration update.
use serde::{Deserialize, Serialize};

/// An enum that defines the target level for a configuration update. This tells
/// the `ConfigProvider` which `settings.json` file or memory layer to modify.
///
/// The integer values are chosen for direct compatibility with VS Code's
/// internal API.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigurationTarget {
	/// Target the user settings file for the local machine.
	UserLocal = 1,
	/// Target the user settings, potentially synced across machines.
	User = 2,
	/// Target the workspace settings file (`.vscode/settings.json`).
	Workspace = 3,
	/// Target a specific folder's settings in a multi-root workspace.
	WorkspaceFolder = 4,
	/// Target the default values (typically read-only).
	Default = 5,
	/// Target the in-memory configuration for the current session only.
	Memory = 6,
	/// Target the policy-enforced configuration (read-only).
	Policy = 7,
}
