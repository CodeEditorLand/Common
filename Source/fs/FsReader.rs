use std::path::PathBuf;

use async_trait::async_trait;

/// @module FsReader
/// @description Defines the abstract service trait for read-only filesystem
/// capabilities.
use super::dto::{FileSystemStatDto, FileTypeDto};
use crate::{environment::Environment, error::CommonError};

/// An abstract service contract for an environment component that can perform
/// read-only filesystem operations.
///
/// This trait is implemented by `MountainEnvironment` and uses `tokio::fs` to
/// fulfill the contract. Separating read operations from write operations
/// allows for more granular and secure dependency injection.
#[async_trait]
pub trait FsReader: Environment + Send + Sync {
	/// Reads the entire content of a file into a byte vector.
	///
	/// @param Path - The `PathBuf` of the file to read.
	/// @returns A `Result` containing the file's content as `Vec<u8>`.
	async fn ReadFile(&self, Path:&PathBuf) -> Result<Vec<u8>, CommonError>;

	/// Reads metadata for a file or directory.
	///
	/// @param Path - The `PathBuf` of the file or directory to stat.
	/// @returns A `Result` containing the `FileSystemStatDto` metadata.
	async fn StatFile(&self, Path:&PathBuf) -> Result<FileSystemStatDto, CommonError>;

	/// Reads directory entries.
	///
	/// @param Path - The `PathBuf` of the directory to read.
	/// @returns A `Result` containing a vector of tuples, where each tuple is
	///   (entry_name, entry_file_type).
	async fn ReadDirectory(&self, Path:&PathBuf) -> Result<Vec<(String, FileTypeDto)>, CommonError>;
}
