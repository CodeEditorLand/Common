use std::sync::Arc;

use serde_json::Value;

/// @module UpdateConfiguration
/// @description Defines the ActionEffect for updating a configuration value in
/// a specific target scope.
use super::{
	ConfigProvider::ConfigProvider,
	dto::{ConfigurationOverridesDto, ConfigurationTarget},
};
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will update a configuration value at
/// a specific key and target scope (e.g., User or Workspace settings).
///
/// It uses the `ConfigProvider` capability from the environment to perform the
/// operation, which will typically involve modifying a `settings.json` file.
///
/// @param Key - The dot-separated configuration key to update.
/// @param ValueToSet - The new `serde_json::Value` to set for the key.
/// @param TargetAsU32 - The integer representation of the `ConfigurationTarget`
/// enum. @param OverridesValue - A DTO specifying scope overrides (e.g., for a
/// specific language). @param ScopeToLanguage - An optional flag related to
/// language-specific settings.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn UpdateConfiguration<Runtime>(
	Key:String,
	ValueToSet:Value,
	TargetAsU32:u32,
	OverridesValue:Value,
	ScopeToLanguage:Option<bool>,
) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn ConfigProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let KeyClone = Key.clone();
		let ValueToSetClone = ValueToSet.clone();
		let OverridesValueClone = OverridesValue.clone();
		let ScopeToLanguageClone = ScopeToLanguage;
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn ConfigProvider> = Environment.Require();

			// Deserialize the integer target into the enum.
			let TargetParsed:ConfigurationTarget = serde_json::from_value(Value::from(TargetAsU32)).map_err(|e| {
				CommonError::InvalidArg {
					ArgumentName:"Target".to_string(),
					Reason:format!("Failed to parse ConfigurationTarget from u32 {}: {}", TargetAsU32, e),
				}
			})?;

			// Deserialize the overrides DTO.
			let OverridesParsed:ConfigurationOverridesDto =
				serde_json::from_value(OverridesValueClone).map_err(|e| {
					CommonError::InvalidArg {
						ArgumentName:"OverridesValue".to_string(),
						Reason:format!("Failed to parse ConfigurationOverridesDto: {}", e),
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
