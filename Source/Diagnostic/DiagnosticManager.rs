//! # DiagnosticManager Trait
//!
//! Defines the abstract service trait for managing diagnostic collections,
//! which represent problems like errors and warnings in source code.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// diagnostic collections.
///
/// Diagnostics are problems detected in the workspace, such as compiler errors
/// or linter warnings. They are typically owned by a source (e.g., a
/// "typescript-linter") and associated with specific resource URIs.
#[async_trait]
pub trait DiagnosticManager: Environment + Send + Sync {
	/// Sets or updates diagnostics for multiple resources from a specific
	/// owner.
	///
	/// # Parameters
	/// * `Owner`: A string identifying the source of the diagnostics.
	/// * `EntriesDTOValue`: A `serde_json::Value` representing an array of
	///   entries. Each entry is a tuple: `[UriComponentsValue,
	///   Option<Vec<MarkerDataDTOAsValue>>]`. To clear diagnostics for a
	///   resource, provide `None` or an empty array for its entry.
	async fn SetDiagnostics(&self, Owner:String, EntriesDTOValue:Value) -> Result<(), CommonError>;

	/// Clears all diagnostics that were previously reported by a specific
	/// owner.
	///
	/// # Parameters
	/// * `Owner`: The identifier of the diagnostic source to clear.
	async fn ClearDiagnostics(&self, Owner:String) -> Result<(), CommonError>;

	/// Retrieves all diagnostics currently in the system, with an option to
	/// filter by a specific resource URI.
	///
	/// # Parameters
	/// * `ResourceURIFilterOption`: An optional `serde_json::Value`
	///   representing a `UriComponents` DTO. If `Some`, only diagnostics for
	///   that URI are returned.
	///
	/// # Returns
	/// A `serde_json::Value` representing an array of tuples:
	/// `[[UriComponentsValue, Vec<MarkerDataDtoAsValue>]]`.
	async fn GetAllDiagnostics(&self, ResourceURIFilterOption:Option<Value>) -> Result<Value, CommonError>;
}
