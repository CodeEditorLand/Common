use std::sync::Arc;

use serde_json::Value;

/// @module InspectConfiguration
/// @description Defines the ActionEffect for inspecting a configuration value
/// from all sources.
use super::{
	ConfigInspector::ConfigInspector,
	dto::{ConfigurationOverridesDto, InspectResultDataDto},
};
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will inspect a configuration key to
/// get its value from all relevant sources (default, user, workspace, etc.).
///
/// It uses the `ConfigInspector` capability from the environment to perform the
/// operation.
///
/// @param Key - The dot-separated configuration key to inspect (e.g.,
/// "editor.fontSize"). @param OverridesValue - A `serde_json::Value`
/// representing the `ConfigurationOverridesDto`,   which can specify a resource
/// or language scope for the inspection.
///
/// @returns An `ActionEffect` that resolves with an
/// `Option<InspectResultDataDto>`, containing   the detailed breakdown of the
/// configuration value from all scopes.
pub fn InspectConfiguration<Runtime>(
	Key:String,
	OverridesValue:Value,
) -> ActionEffect<Arc<Runtime>, CommonError, Option<InspectResultDataDto>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn ConfigInspector>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let KeyClone = Key.clone();
		let OverridesValueClone = OverridesValue.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Inspector:Arc<dyn ConfigInspector> = Environment.Require();

			let OverridesParsed:ConfigurationOverridesDto =
				serde_json::from_value(OverridesValueClone).map_err(|e| {
					CommonError::InvalidArg {
						ArgumentName:"OverridesValue".to_string(),
						Reason:format!("Failed to parse ConfigurationOverridesDto: {}", e),
					}
				})?;

			Inspector.InspectConfigurationValue(KeyClone, OverridesParsed).await
		})
	}))
}
