// File: Common/Source/Configuration/InspectConfiguration.rs
// Role: Defines the `InspectConfiguration` ActionEffect.
// Responsibilities:
//   - Provide a declarative effect for inspecting a configuration's value
//     across all its sources.
//   - This effect abstracts the "what" (inspect a configuration) from the "how"
//     (the ConfigurationInspector implementation).

use std::sync::Arc;

use serde_json::Value;

use super::{
	ConfigurationInspector::ConfigurationInspector,
	DTO::{ConfigurationOverridesDTO::ConfigurationOverridesDTO, InspectResultDataDTO::InspectResultDataDTO},
};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will inspect a configuration key to
/// get its value from all relevant sources (e.g., default, user, workspace).
///
/// It uses the `ConfigurationInspector` capability from the environment to
/// perform the operation.
///
/// # Parameters
///
/// * `Key`: The dot-separated configuration key to inspect (e.g.,
///   "Editor.FontSize").
/// * `OverridesValue`: A `serde_json::Value` representing the
///   `ConfigurationOverridesDTO`, which can specify a resource or language
///   scope for the inspection.
///
/// # Returns
///
/// An `ActionEffect` that resolves with an `Option<InspectResultDataDTO>`,
/// containing the detailed breakdown of the configuration value from all
/// scopes.
pub fn InspectConfiguration(
	Key:String,
	OverridesValue:Value,
) -> ActionEffect<Arc<dyn ConfigurationInspector>, CommonError, Option<InspectResultDataDTO>> {
	ActionEffect::New(Arc::new(move |Inspector:Arc<dyn ConfigurationInspector>| {
		let KeyClone = Key.clone();
		let OverridesValueClone = OverridesValue.clone();
		Box::pin(async move {
			let OverridesParsed:ConfigurationOverridesDTO =
				serde_json::from_value(OverridesValueClone).map_err(|e| {
					CommonError::InvalidArgument {
						ArgumentName:"OverridesValue".to_string(),
						Reason:format!("Failed to parse ConfigurationOverridesDTO: {}", e),
					}
				})?;

			Inspector.InspectConfigurationValue(KeyClone, OverridesParsed).await
		})
	}))
}
