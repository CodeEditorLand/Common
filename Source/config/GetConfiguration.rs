use std::sync::Arc;

use serde_json::Value;

/// @module GetConfiguration
/// @description Defines the ActionEffect for retrieving a configuration value
/// or section.
use super::{ConfigProvider::ConfigProvider, dto::ConfigurationOverridesDto};
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will retrieve the final, merged
/// configuration value for a given section, applying any specified overrides.
///
/// It uses the `ConfigProvider` capability from the environment to perform the
/// operation.
///
/// @param Section - An optional, dot-separated key to a specific configuration
/// section.   If `None`, the entire configuration object is returned.
/// @param OverridesValue - A `serde_json::Value` representing the
/// `ConfigurationOverridesDto`,   which can specify a resource or language
/// scope.
///
/// @returns An `ActionEffect` that resolves with a `serde_json::Value`
/// containing the   requested configuration.
pub fn GetConfiguration<Runtime>(
	Section:Option<String>,
	OverridesValue:Value,
) -> ActionEffect<Arc<Runtime>, CommonError, Value>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn ConfigProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let SectionClone = Section.clone();
		let OverridesValueClone = OverridesValue.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn ConfigProvider> = Environment.Require();

			let OverridesParsed:ConfigurationOverridesDto =
				serde_json::from_value(OverridesValueClone).map_err(|e| {
					CommonError::InvalidArg {
						ArgumentName:"OverridesValue".to_string(),
						Reason:format!("Failed to parse ConfigurationOverridesDto: {}", e),
					}
				})?;

			Provider.GetConfigurationValue(SectionClone, OverridesParsed).await
		})
	}))
}
