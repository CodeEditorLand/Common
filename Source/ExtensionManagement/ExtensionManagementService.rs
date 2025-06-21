// File: Common/Source/ExtensionManagement/ExtensionManagementService.rs
// Role: Defines the abstract service trait for discovering and managing
// extensions. Responsibilities:
//   - Provide a contract for scanning the file system for extensions.
//   - Provide a contract for retrieving information about installed extensions.

//! # ExtensionManagementService Trait
//!
//! Defines the abstract service trait for discovering and managing extensions.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can discover,
/// parse, and provide information about installed extensions.
#[async_trait]
pub trait ExtensionManagementService: Environment + Send + Sync {
	/// Scans the predefined extension directories, parses the `package.json`
	/// for each found extension, and populates the internal state with the
	/// results.
	///
	/// This is typically a long-running operation that should be performed
	/// during application startup.
	async fn ScanForExtensions(&self) -> Result<(), CommonError>;

	/// Retrieves the metadata for all successfully scanned extensions.
	///
	/// # Returns
	///
	/// A `Result` containing a `Vec<Value>`, where each `Value` is a JSON
	/// object representing an extension's `package.json` content.
	async fn GetExtensions(&self) -> Result<Vec<Value>, CommonError>;

	/// Retrieves the metadata for a single extension, identified by its ID
	/// (e.g., "vscode.typescript-language-features").
	///
	/// # Parameters
	/// * `ExtensionID`: The unique identifier of the extension to retrieve.
	///
	/// # Returns
	///
	/// A `Result` containing an `Option<Value>`. `Some(Value)` if the extension
	/// was found, `None` otherwise.
	async fn GetExtension(&self, ExtensionID:String) -> Result<Option<Value>, CommonError>;
}
