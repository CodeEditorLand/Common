// File: Common/Source/Diagnostic/DiagnosticManager.rs
// Role: Defines the abstract service trait for managing diagnostic collections.
// Responsibilities:
//   - Provide a contract for setting, clearing, and retrieving diagnostics.
//   - This trait is the central point of interaction for any component that
//     needs to manage problem markers (errors, warnings, etc.) in the
//     workspace.

//! # DiagnosticManager Trait
//!
//! Defines the abstract service trait for managing diagnostic collections,
//! which represent problems like errors and warnings in source code.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// diagnostic collections.
/// Diagnostics are problems detected in the workspace, such as compiler errors
/// or linter warnings. They are typically owned by a source (e.g., a
/// "typescript-linter") and associated with specific resource URIs.
#[async_trait]
pub trait DiagnosticManager: Environment + Send + Sync {
	/// Sets or updates diagnostics for multiple resources from a specific
	/// owner.
	///
	/// # Parameters
	/// * `Owner`: A string identifying the source of the diagnostics (e.g.,
	///   "cocoon-diag-0-typescript").
	/// * `EntriesDTOValue`: A `serde_json::Value` that deserializes into an
	///   array of tuples. Each tuple has the shape `[UriComponentsDTO,
	///   Option<Vec<MarkerDataDTO>>]`. To clear diagnostics for a resource,
	///   provide `None` or an empty vector for its entry.
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
	///   that specific URI are returned. If `None`, all diagnostics are
	///   returned.
	///
	/// # Returns
	/// A `serde_json::Value` representing an array of tuples:
	/// `Vec<[UriComponentsDTO, Vec<MarkerDataDTO>]>`.
	async fn GetAllDiagnostics(&self, ResourceURIFilterOption:Option<Value>) -> Result<Value, CommonError>;
}
