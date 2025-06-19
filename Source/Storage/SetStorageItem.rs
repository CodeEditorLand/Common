//! # SetStorageItem Effect
//!
//! Defines the `ActionEffect` for setting or updating an item in Memento-style
//! storage.

use std::sync::Arc;

use serde_json::Value;

use super::StorageProvider::StorageProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will set or update an item in either
/// global or workspace-scoped Memento storage.
///
/// It uses the `StorageProvider` capability from the environment to perform the
/// actual data persistence on the host.
///
/// # Parameters
/// * `TargetObject`: A `serde_json::Value` expected to be an object with the
///   following fields:
///     - `Scope` (boolean, optional): `true` for global scope, `false` or
///       absent for workspace scope.
///     - `Key` (string, required): The key of the item to set or update.
/// * `ValueToSet`: The `serde_json::Value` to store. If this value is
///   `Value::Null`, the effect will delete the item from storage.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn SetStorageItem<TRunTime>(TargetObject:Value, ValueToSet:Value) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn StorageProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let TargetObjectClone = TargetObject.clone();
		let ValueToSetClone = ValueToSet.clone();
		Box::pin(async move {
			let IsGlobal = TargetObjectClone.get("Scope").and_then(Value::as_bool).unwrap_or(false);

			let KeyString = TargetObjectClone
				.get("Key")
				.and_then(Value::as_str)
				.ok_or_else(|| {
					CommonError::InvalidArgument {
						ArgumentName:"TargetObject.Key".to_string(),
						Reason:"Expected a 'Key' string field in TargetObject.".to_string(),
					}
				})?
				.to_string();

			// A JSON null from the caller signals deletion to the provider.
			let ValueOption = if ValueToSetClone.is_null() { None } else { Some(ValueToSetClone) };

			let Provider:Arc<dyn StorageProvider> = RunTime.Require();
			Provider.UpdateStorageValue(IsGlobal, KeyString, ValueOption).await
		})
	}))
}
