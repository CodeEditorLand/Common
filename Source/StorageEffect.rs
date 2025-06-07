// Land_Common/src/storage_effects.rs
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value; // For parameters, especially when passed as a single JSON object

// Ensure AppRuntime is the correct type from your runtime module.
use crate::runtime::AppRuntime;
use crate::{
	effect::ActionEffect,
	environment::{Environment, Requires},
	errors::CommonError,
};

/// Trait for an environment component that provides persistent storage
/// capabilities, similar to VS Code's Memento API (global and workspace
/// storage).
#[async_trait]
pub trait StorageProvider: Environment {
	/// Retrieves a value from storage.
	///
	/// # Argument
	/// * `is_global_scope`: If `true`, retrieves from global storage;
	///   otherwise, from workspace storage.
	/// * `key`: The key of the value to retrieve.
	///
	/// # Returns
	/// `Ok(Some(Value))` if the key exists, `Ok(None)` if not, or `Err` on
	/// failure.
	async fn get_storage_value(&self, is_global_scope:bool, key:&str) -> Result<Option<Value>, CommonError>;

	/// Updates or stores a value in storage.
	/// Setting `value_to_set` to `None` (or `Value::Null` via the effect)
	/// typically deletes the key.
	///
	/// # Argument
	/// * `is_global_scope`: If `true`, updates global storage; otherwise,
	///   workspace storage.
	/// * `key`: The key of the value to update.
	/// * `value_to_set`: The `serde_json::Value` to store, or `None` to delete
	///   the key.
	async fn update_storage_value(
		&self,
		is_global_scope:bool,
		key:String,
		value_to_set:Option<Value>,
	) -> Result<(), CommonError>;

	// Potential future extension:
	// async fn get_all_storage_keys(&self, is_global_scope: bool) ->
	// Result<Vec<String>, CommonError>;
}

// --- Effect Constructors ---

/// Creates an effect to retrieve an item from storage.
///
/// # Argument
/// * `target_object`: A `serde_json::Value` expected to be an object with:
///   - `scope` (boolean, optional): `true` for global, `false` or absent for
///     workspace.
///   - `key` (string, required): The key of the item to retrieve.
pub fn get_storage_item(target_object:Value) -> ActionEffect<Arc<AppRuntime>, CommonError, Option<Value>> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let target_obj_clone = target_object.clone();
		Box::pin(async move {
			// Default to workspace scope (is_global = false) if 'scope' is not provided or
			// not a bool.
			let is_global = target_obj_clone.get("scope").and_then(Value::as_bool).unwrap_or(false);
			let key_str = target_obj_clone
				.get("key")
				.and_then(Value::as_str)
				.ok_or_else(|| {
					CommonError::InvalidArg(
						"target_object.key".to_string(),
						"Expected a 'key' string field in target_object.".to_string(),
					)
				})?
				.to_string();

			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn StorageProvider + Send + Sync> = concrete_env.require();
			provider.get_storage_value(is_global, &key_str).await
		})
	}))
}

/// Creates an effect to set or update an item in storage.
///
/// # Argument
/// * `target_object`: A `serde_json::Value` expected to be an object with:
///   - `scope` (boolean, optional): `true` for global, `false` or absent for
///     workspace.
///   - `key` (string, required): The key of the item to set/update.
/// * `value_to_set`: The `serde_json::Value` to store. If `Value::Null`, the
///   key is typically deleted.
pub fn set_storage_item(target_object:Value, value_to_set:Value) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let target_obj_clone = target_object.clone();
		let value_to_set_clone = value_to_set.clone();
		Box::pin(async move {
			let is_global = target_obj_clone.get("scope").and_then(Value::as_bool).unwrap_or(false);
			let key_str = target_obj_clone
				.get("key")
				.and_then(Value::as_str)
				.ok_or_else(|| {
					CommonError::InvalidArg(
						"target_object.key".to_string(),
						"Expected a 'key' string field in target_object.".to_string(),
					)
				})?
				.to_string();

			// If value_to_set is JSON null, interpret as a request to delete the item.
			let value_opt = if value_to_set_clone.is_null() { None } else { Some(value_to_set_clone) };

			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn StorageProvider + Send + Sync> = concrete_env.require();
			provider.update_storage_value(is_global, key_str, value_opt).await
		})
	}))
}
