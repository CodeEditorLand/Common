use std::sync::Arc;

use serde_json::Value;

/// @module GetStorageItem
/// @description Defines the ActionEffect for retrieving an item from
/// Memento-style storage.
use super::StorageProvider::StorageProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will retrieve an item from either
/// global or workspace-scoped storage.
///
/// It uses the `StorageProvider` capability from the environment to perform the
/// actual data retrieval from the host's persistent storage.
///
/// @param TargetObject - A `serde_json::Value` expected to be an object with
/// the following fields:
///   - `Scope` (boolean, optional): `true` for global scope, `false` or absent
///     for workspace scope.
///   - `Key` (string, required): The key of the item to retrieve.
///
/// @returns An `ActionEffect` that resolves with an `Option<Value>`, containing
/// the   retrieved value or `None` if the key does not exist. It fails with a
/// `CommonError`   if the `TargetObject` is malformed.
pub fn GetStorageItem<Runtime>(TargetObject:Value) -> ActionEffect<Arc<Runtime>, CommonError, Option<Value>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn StorageProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let TargetObjectClone = TargetObject.clone();
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

			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn StorageProvider> = Environment.Require();
			Provider.GetStorageValue(IsGlobal, &KeyString).await
		})
	}))
}
