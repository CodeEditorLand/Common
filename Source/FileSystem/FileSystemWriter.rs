//! # FileSystemWriter Trait
//!
//! Defines the abstract service trait for write and modification filesystem
//! capabilities.

use std::path::PathBuf;

use async_trait::async_trait;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can perform
/// write and modification filesystem operations.
///
/// Implemented by `MountainEnvironment` and typically uses `tokio::fs` to
/// fulfill the contract. Separating write operations from read operations
/// enhances security by allowing capabilities to be granted on a
/// need-to-know basis.
#[async_trait]
pub trait FileSystemWriter: Environment + Send + Sync {
	/// Writes byte content to a file.
	///
	/// # Parameters
	/// * `Path`: The path of the file to write to.
	/// * `Content`: The byte vector to write.
	/// * `Create`: If `true`, the file will be created if it does not exist.
	/// * `Overwrite`: If `true`, an existing file will be overwritten.
	async fn WriteFile(&self, Path:&PathBuf, Content:Vec<u8>, Create:bool, Overwrite:bool) -> Result<(), CommonError>;

	/// Creates a directory at the specified path.
	///
	/// # Parameters
	/// * `Path`: The path of the directory to create.
	/// * `Recursive`: If `true`, creates all parent directories as needed.
	async fn CreateDirectory(&self, Path:&PathBuf, Recursive:bool) -> Result<(), CommonError>;

	/// Deletes a file or directory.
	///
	/// # Parameters
	/// * `Path`: The path of the item to delete.
	/// * `Recursive`: If `true`, deletes a directory and all its contents.
	/// * `UseTrash`: If `true`, moves the item to the system's trash or
	///   recycling bin instead of permanently deleting it.
	async fn Delete(&self, Path:&PathBuf, Recursive:bool, UseTrash:bool) -> Result<(), CommonError>;

	/// Renames (moves) a file or directory.
	///
	/// # Parameters
	/// * `Source`: The original path of the item.
	/// * `Target`: The new path for the item.
	/// * `Overwrite`: If `true`, an existing item at the target path will be
	///   overwritten.
	async fn Rename(&self, Source:&PathBuf, Target:&PathBuf, Overwrite:bool) -> Result<(), CommonError>;

	/// Copies a file or directory.
	///
	/// # Parameters
	/// * `Source`: The path of the item to copy.
	/// * `Target`: The destination path for the copy.
	/// * `Overwrite`: If `true`, an existing item at the target path will be
	///   overwritten.
	async fn Copy(&self, Source:&PathBuf, Target:&PathBuf, Overwrite:bool) -> Result<(), CommonError>;

	/// Creates a new, empty file at the specified path. This is a convenience
	/// method that will fail if the file already exists.
	async fn CreateFile(&self, Path:&PathBuf) -> Result<(), CommonError>;
}
