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
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will update a configuration value at
/// a specific key and target scope (e.g., User or WorkSpace settings).
///
/// It uses the `ConfigurationProvider` capability from the environment to
/// perform the operation, which will typically involve modifying a
/// `settings.json` file on disk.
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
pub fn UpdateConfiguration<TRunTime>(
	Key:String,
	ValueToSet:Value,
	TargetAsU32:u32,
	OverridesValue:Value,
	ScopeToLanguage:Option<bool>,
) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn ConfigurationProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let KeyClone = Key.clone();
		let ValueToSetClone = ValueToSet.clone();
		let OverridesValueClone = OverridesValue.clone();
		let ScopeToLanguageClone = ScopeToLanguage;
		Box::pin(async move {
			let Provider:Arc<dyn ConfigurationProvider> = RunTime.Require();

			// Deserialize the integer target into the enum.
			let TargetParsed:ConfigurationTarget = serde_json::from_value(Value::from(TargetAsU32)).map_err(|e| {
				CommonError::InvalidArgument {
					ArgumentName:"Target".to_string(),
					Reason:format!("Failed to parse ConfigurationTarget from u32 {}: {}", TargetAsU32, e),
				}
			})?;

			// Deserialize the overrides DTO.
			let OverridesParsed:ConfigurationOverridesDTO =
				serde_json::from_value(OverridesValueClone).map_err(|e| {
					CommonError::InvalidArgument {
						ArgumentName:"OverridesValue".to_string(),
						Reason:format!("Failed to parse ConfigurationOverridesDTO: {}", e),
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
