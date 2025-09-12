//! # ConfigurationInspector Trait
//!
//! Defines the abstract service trait for inspecting the sources of
//! configuration values.

use async_trait::async_trait;

use super::DTO::{ConfigurationOverridesDTO::ConfigurationOverridesDTO, InspectResultDataDTO::InspectResultDataDTO};
use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can inspect
/// a configuration key to provide details about its value in all relevant
/// scopes (e.g., default, user, workspace) and its final effective value.
///
/// This capability is used to power UIs like the "Settings" editor, which
/// often shows where a particular setting is defined and allows the user to
/// see inherited values.
#[async_trait]
pub trait ConfigurationInspector: Environment + Send + Sync {
	/// Inspects a configuration key to get its value from all relevant scopes.
	///
	/// # Parameters
	///
	/// * `Key`: The dot-separated configuration key to inspect.
	/// * `Overrides`: A DTO specifying scope overrides (e.g., for a specific
	///   resource or language).
	///
	/// # Returns
	///
	/// A `Result` containing an `Option<InspectResultDataDTO>`, which holds
	/// the detailed breakdown of the key's values across all scopes. Returns
	/// `None` if the key is not found in any configuration source.
	async fn InspectConfigurationValue(
		&self,

		Key:String,

		Overrides:ConfigurationOverridesDTO,
	) -> Result<Option<InspectResultDataDTO>, CommonError>;
}
