use std::sync::Arc;

use serde_json::Value;

/// @module SetStorageItem
/// @description Defines the ActionEffect for setting or updating an item in
/// Memento-style storage.
use super::StorageProvider::StorageProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will set or update an item in either
/// global or workspace-scoped storage.
///
/// It uses the `StorageProvider` capability from the environment to perform the
/// actual data persistence on the host.
///
/// @param TargetObject - A `serde_json::Value` expected to be an object with
/// the following fields:
///   - `Scope` (boolean, optional): `true` for global scope, `false` or absent
///     for workspace scope.
///   - `Key` (string, required): The key of the item to set or update.
///
/// @param ValueToSet - The `serde_json::Value` to store. If this value is
/// `Value::Null`,   the effect will delete the item from storage.
///
/// @returns An `ActionEffect` that resolves to `()` on success or fails with a
/// `CommonError`.
pub fn SetStorageItem<Runtime>(TargetObject:Value, ValueToSet:Value) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn StorageProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let TargetObjectClone = TargetObject.clone();
		let ValueToSetClone = ValueToSet.clone();
		Box::pin(async move {
			let IsGlobal = TargetObjectClone.get("Scope").and_then(Value::as_bool).unwrap_or(false);

			let KeyString = TargetObjectClone
				.get("Key")
				.and_then(Value::as_str)
				.ok_or_else(|| {
					CommonError::InvalidArg {
						ArgumentName:"TargetObject.Key".to_string(),
						Reason:"Expected a 'Key' string field in TargetObject.".to_string(),
					}
				})?
				.to_string();

			// Convert a JSON null into a Rust None, which signals deletion to the provider.
			let ValueOption = if ValueToSetClone.is_null() { None } else { Some(ValueToSetClone) };

			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn StorageProvider> = Environment.Require();
			Provider.UpdateStorageValue(IsGlobal, KeyString, ValueOption).await
		})
	}))
}
