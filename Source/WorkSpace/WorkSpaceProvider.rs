//! # WorkSpaceProvider Trait
//!
//! Defines the abstract service trait for querying and managing the
//! application's workspace.

use std::path::PathBuf;

use async_trait::async_trait;
use serde_json::Value;
use url::Url;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can provide
/// information about the current workspace.
///
/// This trait is the primary interface for interacting with workspace folders,
/// configuration paths, trust settings, and for performing workspace-wide
/// operations like finding files.
#[async_trait]
pub trait WorkSpaceProvider: Environment + Send + Sync {
	/// Retrieves information about all currently open workspace folders.
	///
	/// # Returns
	/// A `Result` containing a vector of tuples, where each tuple is
	/// `(FolderURI, FolderName, FolderIndex)`.
	async fn GetWorkSpaceFoldersInfo(&self) -> Result<Vec<(Url, String, usize)>, CommonError>;

	/// Retrieves information for the specific workspace folder that contains
	/// the given URI.
	///
	/// # Parameters
	/// * `URIToMatch`: The URI to find the containing folder for.
	async fn GetWorkSpaceFolderInfo(&self, URIToMatch:Url) -> Result<Option<(Url, String, usize)>, CommonError>;

	/// Gets the name of the current workspace.
	async fn GetWorkSpaceName(&self) -> Result<Option<String>, CommonError>;

	/// Gets the path to the workspace configuration file (e.g., a
	/// `.code-workspace` file), if one exists.
	async fn GetWorkSpaceConfigurationPath(&self) -> Result<Option<PathBuf>, CommonError>;

	/// Checks if the current workspace is trusted by the user.
	async fn IsWorkSpaceTrusted(&self) -> Result<bool, CommonError>;

	/// Prompts the user to grant or deny trust to the current workspace.
	///
	/// # Parameters
	/// * `Options`: Optional DTO with further information for the trust prompt.
	async fn RequestWorkSpaceTrust(&self, Options:Option<Value>) -> Result<bool, CommonError>;

	/// Finds files within the workspace matching the given criteria.
	///
	/// # Parameters
	/// * `IncludePatternDTO`: A DTO representing the glob pattern to include.
	/// * `ExcludePatternDTO`: An optional DTO for files/folders to exclude.
	/// * `MaxResults`: An optional limit on the number of results to return.
	/// * `UseIgnoreFiles`: Whether to respect `.gitignore`-style ignore files.
	/// * `FollowSymlinks`: Whether to follow symbolic links during the search.
	async fn FindFilesInWorkSpace(
		&self,
		IncludePatternDTO:Value,
		ExcludePatternDTO:Option<Value>,
		MaxResults:Option<usize>,
		UseIgnoreFiles:bool,
		FollowSymlinks:bool,
	) -> Result<Vec<Url>, CommonError>;

	/// Requests that the host application open the specified file path in an
	/// editor.
	async fn OpenFile(&self, Path:PathBuf) -> Result<(), CommonError>;
}
