//! # FileSystemReader Trait
//!
//! Defines the abstract service trait for read-only filesystem capabilities.

use std::path::PathBuf;

use async_trait::async_trait;

use super::DTO::{FileSystemStatDTO::FileSystemStatDTO, FileTypeDTO::FileTypeDTO};
use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can perform
/// read-only filesystem operations.
///
/// Implemented by `MountainEnvironment` and typically uses `tokio::fs` to
/// fulfill the contract. Separating read operations from write operations
/// allows for more granular and secure dependency injection, as some parts of
/// the application may only need read access.
#[async_trait]
pub trait FileSystemReader: Environment + Send + Sync {
	/// Reads the entire content of a file into a byte vector.
	///
	/// # Parameters
	/// * `Path`: The `PathBuf` of the file to read.
	///
	/// # Returns
	/// A `Result` containing the file's content as `Vec<u8>`.
	async fn ReadFile(&self, Path:&PathBuf) -> Result<Vec<u8>, CommonError>;

	/// Reads metadata for a file or directory.
	///
	/// # Parameters
	/// * `Path`: The `PathBuf` of the file or directory to stat.
	///
	/// # Returns
	/// A `Result` containing the `FileSystemStatDTO` metadata.
	async fn StatFile(&self, Path:&PathBuf) -> Result<FileSystemStatDTO, CommonError>;

	/// Reads the entries of a directory.
	///
	/// # Parameters
	/// * `Path`: The `PathBuf` of the directory to read.
	///
	/// # Returns
	/// A `Result` containing a vector of tuples, where each tuple is
	/// `(entry_name, entry_file_type)`.
	async fn ReadDirectory(&self, Path:&PathBuf) -> Result<Vec<(String, FileTypeDTO)>, CommonError>;
}
