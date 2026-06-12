// File: Common/Source/Storage/SetStorageItem.rs
// Role: Defines the `SetStorageItem` ActionEffect.
// Responsibilities:
//   - Provide a declarative effect for setting or updating a single item in
//     Memento storage.
//   - This effect abstracts the "what" (set a storage item) from the "how" (the
//     StorageProvider implementation).
//
// NOTE: This effect is part of a legacy, per-key storage model. The newer,

// high-performance storage model in Cocoon uses a batch-oriented approach
// (`GetAllStorage`, `SetAllStorage`), making this effect obsolete for that
// use case. It is kept for potential other uses or until fully deprecated.

//! # SetStorageItem Effect
//!
//! Defines the `ActionEffect` for setting or updating an item in Memento-style
//! storage.

use std::sync::Arc;

use serde_json::Value;

use super::StorageProvider::StorageProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will set or update an item in either
/// global or workspace-scoped Memento storage.
/// It uses the `StorageProvider` capability from the environment to perform the
/// actual data persistence on the host.
///
/// # Parameters
/// * `TargetObjectValue`: A `serde_json::Value` expected to be an object with
///   the following fields:
///     - `Scope` (boolean, optional): `true` for global scope, `false` or
///       absent for workspace scope.
///     - `Key` (string, required): The key of the item to set or update.
/// * `ValueToSet`: The `serde_json::Value` to store. If this value is
///   `Value::Null`, the effect will delete the item from storage.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn SetStorageItem(
	TargetObjectValue:Value,

	ValueToSet:Value,
) -> ActionEffect<Arc<dyn StorageProvider>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn StorageProvider>| {
		let TargetObjectClone = TargetObjectValue.clone();

		let ValueToSetClone = ValueToSet.clone();

		Box::pin(async move {
			let IsGlobal = TargetObjectClone.get("Scope").and_then(Value::as_bool).unwrap_or(false);

			let Key = TargetObjectClone
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
			let ValueToStore = if ValueToSetClone.is_null() { None } else { Some(ValueToSetClone) };

			Provider.UpdateStorageValue(IsGlobal, Key, ValueToStore).await
		})
	}))
}
