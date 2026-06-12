//! # ConfigurationScope DTO
//!
//! Defines the Data Transfer Object enum for representing the scope at which a
//! configuration setting can be applied.

use serde::{Deserialize, Serialize};

/// An enum that describes the scope of a configuration value, as typically
/// defined in an extension's `package.json` manifest.
/// This determines where the setting can be configured by a user (e.g., in
/// User settings, Workspace settings, or both). The integer values are chosen
/// for direct compatibility with VS Code's internal API.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigurationScope {
	/// Application-specific configuration, which can only be configured in user
	/// settings.
	Application = 1,

	/// Machine-specific configuration, which can only be configured in user
	/// settings and is not synced.
	Machine = 2,

	/// Window-specific configuration, which can be configured in user or
	/// workspace settings.
	Window = 3,

	/// Resource-specific configuration, which can be configured in all settings
	/// levels (user, workspace, folder).
	Resource = 4,

	/// Language-specific configuration, which can be configured in all settings
	/// levels and can be overridden on a per-language basis.
	LanguageDefined = 5,

	/// Machine-specific configuration that can be overridden by workspace
	/// settings.
	MachineOverridable = 6,
}
