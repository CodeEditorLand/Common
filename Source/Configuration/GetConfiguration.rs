// File: Common/Source/Configuration/GetConfiguration.rs
// Role: Defines the `GetConfiguration` ActionEffect.
// Responsibilities:
//   - Provide a declarative effect for retrieving a merged configuration value
//     or section.
//   - This effect abstracts the "what" (get a configuration) from the "how"
//     (the ConfigurationProvider implementation).

//! # GetConfiguration Effect
//!
//! Defines a declarative `ActionEffect` for retrieving a merged configuration
//! value or section from the `ConfigurationProvider`.

use std::sync::Arc;

use serde_json::Value;

use super::{ConfigurationProvider::ConfigurationProvider, DTO::ConfigurationOverridesDTO::ConfigurationOverridesDTO};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will retrieve the final, merged
/// configuration value for a given section, applying any specified overrides.
///
/// It uses the `ConfigurationProvider` capability from the environment to
/// perform the operation.
///
/// # Parameters
///
/// * `Section`: An optional, dot-separated key to a specific configuration
///   section. If `None`, the entire configuration object is returned.
/// * `OverridesValue`: A `serde_json::Value` representing the
///   `ConfigurationOverridesDTO`, which can specify a resource or language
///   scope.
///
/// # Returns
///
/// An `ActionEffect` that resolves with a `serde_json::Value` containing the
/// requested configuration.
pub fn GetConfiguration(
	Section:Option<String>,

	OverridesValue:Value,
) -> ActionEffect<Arc<dyn ConfigurationProvider>, CommonError, Value> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn ConfigurationProvider>| {
		let SectionClone = Section.clone();

		let OverridesValueClone = OverridesValue.clone();

		Box::pin(async move {
			let OverridesParsed:ConfigurationOverridesDTO =
				serde_json::from_value(OverridesValueClone).map_err(|Error| {
					CommonError::InvalidArgument {
						ArgumentName:"OverridesValue".to_string(),

						Reason:format!("Failed to parse ConfigurationOverridesDTO: {}", Error),
					}
				})?;

			Provider.GetConfigurationValue(SectionClone, OverridesParsed).await
		})
	}))
}
