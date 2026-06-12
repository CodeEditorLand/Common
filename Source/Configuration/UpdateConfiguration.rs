// File: Common/Source/Configuration/UpdateConfiguration.rs
// Role: Defines the `UpdateConfiguration` ActionEffect.
// Responsibilities:
//   - Provide a declarative effect for updating a configuration value in a
//     specific target scope.
//   - This effect abstracts the "what" (update a configuration) from the "how"
//     (the ConfigurationProvider implementation).

//! # UpdateConfiguration Effect
//!
//! Defines the `ActionEffect` for updating a configuration value in a specific
//! target scope.

use std::sync::Arc;

use serde_json::Value;

use super::{
	ConfigurationProvider::ConfigurationProvider,
	DTO::{ConfigurationOverridesDTO::ConfigurationOverridesDTO, ConfigurationTarget::ConfigurationTarget},
};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will update a configuration value at
/// a specific key and target scope (e.g., User or Workspace settings).
///
/// Uses the `ConfigurationProvider` capability from the environment to perform
/// the operation, which will typically involve modifying a `settings.json` file
/// on disk.
///
/// # Parameters
///
/// * `Key`: The dot-separated configuration key to update.
/// * `ValueToSet`: The new `serde_json::Value` to set for the key.
/// * `TargetAsU32`: The integer representation of the `ConfigurationTarget`
///   enum, used for cross-language serialization.
/// * `OverridesValue`: A DTO specifying scope overrides (e.g., for a specific
///   language).
/// * `ScopeToLanguage`: An optional flag related to language-specific settings.
///
/// # Returns
///
/// An `ActionEffect` that resolves to `()` on success.
pub fn UpdateConfiguration(
	Key:String,

	ValueToSet:Value,

	TargetAsU32:u32,

	OverridesValue:Value,

	ScopeToLanguage:Option<bool>,
) -> ActionEffect<Arc<dyn ConfigurationProvider>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn ConfigurationProvider>| {
		let KeyClone = Key.clone();

		let ValueToSetClone = ValueToSet.clone();

		let OverridesValueClone = OverridesValue.clone();

		let ScopeToLanguageClone = ScopeToLanguage;

		Box::pin(async move {
			// Deserialize the integer target into the enum.
			let TargetParsed:ConfigurationTarget =
				serde_json::from_value(Value::from(TargetAsU32)).map_err(|Error| {
					CommonError::InvalidArgument {
						ArgumentName:"Target".to_string(),

						Reason:format!("Failed to parse ConfigurationTarget from u32 {}: {}", TargetAsU32, Error),
					}
				})?;

			// Deserialize the overrides DTO.
			let OverridesParsed:ConfigurationOverridesDTO =
				serde_json::from_value(OverridesValueClone).map_err(|Error| {
					CommonError::InvalidArgument {
						ArgumentName:"OverridesValue".to_string(),

						Reason:format!("Failed to parse ConfigurationOverridesDTO: {}", Error),
					}
				})?;

			Provider
				.UpdateConfigurationValue(
					KeyClone,
					ValueToSetClone,
					TargetParsed,
					OverridesParsed,
					ScopeToLanguageClone,
				)
				.await
		})
	}))
}
