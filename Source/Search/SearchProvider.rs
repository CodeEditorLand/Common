// File: Common/Source/Search/SearchProvider.rs
// Role: Defines the abstract service trait for workspace-wide search
// functionality. Responsibilities:
//   - Provide a contract for performing text searches across the workspace.
//   - (Future) Provide a contract for file searches (finding files by name).

//! # SearchProvider Trait
//!
//! Defines the abstract service trait for performing workspace-wide text and
//! file searches.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can perform
/// high-performance text and file searches across the workspace.
#[async_trait]
pub trait SearchProvider: Environment + Send + Sync {
	/// Performs a text search across all workspace folders.
	///
	/// # Parameters
	/// * `QueryValue`: A DTO representing the text search query, including the
	///   pattern, case sensitivity, etc.
	/// * `OptionsValue`: A DTO representing search options, such as files to
	///   include or exclude.
	///
	/// # Returns
	/// A `Result` containing a `Value` that is a JSON array of `FileMatchDTO`s.
	async fn TextSearch(&self, QueryValue:Value, OptionsValue:Value) -> Result<Value, CommonError>;
}
