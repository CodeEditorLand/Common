use async_trait::async_trait;

/// @module ConfigInspector
/// @description Defines the abstract service trait for inspecting the sources
/// of configuration values.
use super::dto::{ConfigurationOverridesDto, InspectResultDataDto};
use crate::{environment::Environment, error::CommonError};

/// An abstract service contract for an environment component that can inspect
/// a configuration key to provide details about its value in all relevant
/// scopes (e.g., default, user, workspace) and its final effective value.
///
/// This is used to power the "Settings" UI, which often shows where a
/// particular setting is defined.
#[async_trait]
pub trait ConfigInspector: Environment + Send + Sync {
	/// Inspects a configuration key to get its value from all relevant scopes.
	///
	/// @param Key - The dot-separated configuration key to inspect.
	/// @param Overrides - A DTO specifying scope overrides (e.g., for a
	/// specific resource).
	///
	/// @returns A `Result` containing an `Option<InspectResultDataDto>`, which
	/// holds   the detailed breakdown of the key's values across all scopes.
	async fn InspectConfigurationValue(
		&self,
		Key:String,
		Overrides:ConfigurationOverridesDto,
	) -> Result<Option<InspectResultDataDto>, CommonError>;
}
