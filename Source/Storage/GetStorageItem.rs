// File: Common/Source/Storage/GetStorageItem.rs
// Role: Defines the `GetStorageItem` ActionEffect.
// Responsibilities:
//   - Provide a declarative effect for retrieving a single item from Memento
//     storage.
//   - This effect abstracts the "what" (get a storage item) from the "how" (the
//     StorageProvider implementation).
//
// NOTE: This effect is part of a legacy, per-key storage model. The newer,
// high-performance storage model in Cocoon uses a batch-oriented approach
// (`GetAllStorage`, `SetAllStorage`), making this effect obsolete for that
// use case. It is kept for potential other uses or until fully deprecated.

//! # GetStorageItem Effect
//!
//! Defines the `ActionEffect` for retrieving an item from Memento-style
//! storage.

use std::sync::Arc;

use serde_json::Value;

use super::StorageProvider::StorageProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will retrieve an item from either
/// global or workspace-scoped Memento storage.
///
/// It uses the `StorageProvider` capability from the environment to perform the
/// actual data retrieval from the host's persistent storage.
///
/// # Parameters
/// * `TargetObjectValue`: A `serde_json::Value` expected to be an object with
///   the following fields:
///     - `Scope` (boolean, optional): `true` for global scope, `false` or
///       absent for workspace scope.
///     - `Key` (string, required): The key of the item to retrieve.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<Value>`, containing the
/// retrieved value or `None` if the key does not exist.
pub fn GetStorageItem(TargetObjectValue:Value) -> ActionEffect<Arc<dyn StorageProvider>, CommonError, Option<Value>> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn StorageProvider>| {
		let TargetObjectClone = TargetObjectValue.clone();
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

			Provider.GetStorageValue(IsGlobal, &KeyString).await
		})
	}))
}
