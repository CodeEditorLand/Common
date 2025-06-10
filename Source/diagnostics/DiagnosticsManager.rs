use async_trait::async_trait;
use serde_json::Value;

/// @module DiagnosticsManager
/// @description Defines the abstract service trait for managing diagnostic
/// collections, which represent problems like errors and warnings in the code.
use crate::environment::Environment;
use crate::error::CommonError;

/// An abstract service contract for an environment component that can manage
/// diagnostic collections.
///
/// Diagnostics are problems detected in the workspace, such as compiler errors
/// or linter warnings. They are typically owned by a source (e.g.,
/// 'typescript-linter') and associated with specific resource URIs.
#[async_trait]
pub trait DiagnosticsManager: Environment + Send + Sync {
	/// Sets or updates diagnostics for multiple resources from a specific
	/// owner. To clear diagnostics for a resource, provide `None` for its
	/// entry.
	///
	/// @param Owner - A string identifying the source of the diagnostics.
	/// @param EntriesDtoValue - A `serde_json::Value` representing an array of
	/// entries.   Each entry is a tuple: `[UriComponentsValue,
	/// Option<Vec<MarkerDataDtoAsValue>>]`.
	async fn SetDiagnostics(&self, Owner:String, EntriesDtoValue:Value) -> Result<(), CommonError>;

	/// Clears all diagnostics that were previously reported by a specific
	/// owner.
	///
	/// @param Owner - The identifier of the diagnostic source to clear.
	async fn ClearDiagnostics(&self, Owner:String) -> Result<(), CommonError>;

	/// Retrieves all diagnostics currently in the system, with an option to
	/// filter by a specific resource URI.
	///
	/// @param ResourceUriFilterOption - An optional `serde_json::Value`
	/// representing a   `UriComponents` DTO. If `Some`, only diagnostics for
	/// that URI are returned.
	///
	/// @returns A `serde_json::Value` representing an array of tuples:
	///   `[[UriComponentsValue, Vec<MarkerDataDtoAsValue>]]`.
	async fn GetAllDiagnostics(&self, ResourceUriFilterOption:Option<Value>) -> Result<Value, CommonError>;
}
