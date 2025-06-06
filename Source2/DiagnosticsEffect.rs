// Land_Common/src/diagnostics_effects.rs
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value; // For flexible DTO structures passed as arguments/return values

// Ensure AppRuntime is the correct type from your runtime module.
use crate::runtime::AppRuntime;
use crate::{
	effect::ActionEffect,
	environment::{Environment, Requires},
	errors::CommonError,
};
// MarkerDataDto is central to diagnostics and is assumed to be defined in
// language_feature_effects. If it were defined here, it would be: pub use
// crate::language_feature_effects::MarkerDataDto; For now, the environment
// implementation will handle the Value structure.

/// Trait for managing diagnostic collections (errors, warnings, etc.) in the
/// application.
///
/// Diagnostics are typically owned by a source (e.g., a linter extension, a
/// compiler) and associated with specific resource URIs.
#[async_trait]
pub trait DiagnosticsManager: Environment {
	/// Sets or updates diagnostics for multiple resources from a specific
	/// owner.
	///
	/// # Arguments
	/// * `owner`: A string identifying the source of the diagnostics (e.g.,
	///   "eslint", "typescript-language-features").
	/// * `entries_dto_val`: A `serde_json::Value` representing an array of
	///   entries. Each entry is a tuple: `[UriComponentsValue,
	///   Option<Vec<MarkerDataDtoAsValue>>]`.
	///   - `UriComponentsValue`: A JSON Value representing the `UriComponents`
	///     DTO for the resource.
	///   - `Option<Vec<MarkerDataDtoAsValue>>`: An optional array of
	///     `MarkerDataDto` (as JSON Value) for that URI. If `None` or an empty
	///     `Vec`, all diagnostics from this `owner` for that URI are cleared.
	async fn set_diagnostics(&self, owner:String, entries_dto_val:Value) -> Result<(), CommonError>;

	/// Clears all diagnostics from a specific owner.
	/// If no owner is specified (e.g. None, or specific handling), it might
	/// clear all diagnostics. For now, requires an owner.
	async fn clear_diagnostics(&self, owner:String) -> Result<(), CommonError>;

	/// Retrieves all diagnostics, optionally filtered by a resource URI.
	///
	/// # Arguments
	/// * `resource_uri_filter_opt`: An optional `serde_json::Value`
	///   representing `UriComponents` DTO. If `Some`, only diagnostics for that
	///   specific URI are returned. If `None`, all diagnostics from all owners
	///   for all URIs are returned.
	///
	/// # Returns
	/// A `serde_json::Value` representing an array of tuples:
	/// `[[UriComponentsValue, Vec<MarkerDataDtoAsValue>]]`.
	/// Each tuple contains the URI and an array of its associated diagnostics.
	async fn get_all_diagnostics(&self, resource_uri_filter_opt:Option<Value>) -> Result<Value, CommonError>;
}

// --- Effect Constructors ---

/// Creates an effect to set or update diagnostics.
pub fn set_diagnostics(owner:String, entries_dto_val:Value) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let owner_clone = owner.clone();
		let entries_clone = entries_dto_val.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let manager:Arc<dyn DiagnosticsManager + Send + Sync> = concrete_env.require();
			manager.set_diagnostics(owner_clone, entries_clone).await
		})
	}))
}

/// Creates an effect to clear all diagnostics for a given owner.
pub fn clear_diagnostics(owner:String) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let owner_clone = owner.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let manager:Arc<dyn DiagnosticsManager + Send + Sync> = concrete_env.require();
			manager.clear_diagnostics(owner_clone).await
		})
	}))
}

/// Creates an effect to retrieve all diagnostics, optionally filtered by URI.
pub fn get_all_diagnostics(resource_uri_filter_opt:Option<Value>) -> ActionEffect<Arc<AppRuntime>, CommonError, Value> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let filter_clone = resource_uri_filter_opt.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let manager:Arc<dyn DiagnosticsManager + Send + Sync> = concrete_env.require();
			manager.get_all_diagnostics(filter_clone).await
		})
	}))
}
