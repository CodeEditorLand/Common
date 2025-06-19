//! # GetConfiguration Effect
//!
//! Defines the `ActionEffect` for retrieving a configuration value or an entire
//! configuration section.

use std::sync::Arc;

use serde_json::Value;

use super::{ConfigurationProvider::ConfigurationProvider, DTO::ConfigurationOverridesDTO::ConfigurationOverridesDTO};
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

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
pub fn GetConfiguration<TRunTime>(
	Section:Option<String>,
	OverridesValue:Value,
) -> ActionEffect<Arc<TRunTime>, CommonError, Value>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn ConfigurationProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let SectionClone = Section.clone();
		let OverridesValueClone = OverridesValue.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Provider:Arc<dyn ConfigurationProvider> = Environment.Require();

			let OverridesParsed:ConfigurationOverridesDTO =
				serde_json::from_value(OverridesValueClone).map_err(|e| {
					CommonError::InvalidArgument {
						ArgumentName:"OverridesValue".to_string(),
						Reason:format!("Failed to parse ConfigurationOverridesDTO: {}", e),
					}
				})?;

			Provider.GetConfigurationValue(SectionClone, OverridesParsed).await
		})
	}))
}
