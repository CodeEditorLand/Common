//! # GetStorageItem Effect
//!
//! Defines the `ActionEffect` for retrieving an item from Memento-style
//! storage.

use std::sync::Arc;

use serde_json::Value;

use super::StorageProvider::StorageProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will retrieve an item from either
/// global or workspace-scoped Memento storage.
///
/// It uses the `StorageProvider` capability from the environment to perform the
/// actual data retrieval from the host's persistent storage.
///
/// # Parameters
/// * `TargetObject`: A `serde_json::Value` expected to be an object with the
///   following fields:
///     - `Scope` (boolean, optional): `true` for global scope, `false` or
///       absent for workspace scope.
///     - `Key` (string, required): The key of the item to retrieve.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<Value>`, containing the
/// retrieved value or `None` if the key does not exist.
pub fn GetStorageItem<TRunTime>(TargetObject:Value) -> ActionEffect<Arc<TRunTime>, CommonError, Option<Value>>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn StorageProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let TargetObjectClone = TargetObject.clone();
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

			let Provider:Arc<dyn StorageProvider> = RunTime.Require();
			Provider.GetStorageValue(IsGlobal, &KeyString).await
		})
	}))
}
