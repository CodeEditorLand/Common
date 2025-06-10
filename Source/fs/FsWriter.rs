use std::path::PathBuf;

use async_trait::async_trait;

/// @module FsWriter
/// @description Defines the abstract service trait for write and modification
/// filesystem capabilities.
use crate::{environment::Environment, error::CommonError};

/// An abstract service contract for an environment component that can perform
/// write and modification filesystem operations.
///
/// This trait is implemented by `MountainEnvironment` and uses `tokio::fs` to
/// fulfill the contract. Separating write operations from read operations
/// allows for more granular and secure dependency injection, as some parts of
/// the application may only need read access.
#[async_trait]
pub trait FsWriter: Environment + Send + Sync {
	/// Writes byte content to a file.
	///
	/// @param Path - The path of the file to write to.
	/// @param Content - The byte vector to write.
	/// @param Create - If `true`, the file will be created if it does not
	/// exist. @param Overwrite - If `true`, an existing file will be
	/// overwritten.
	async fn WriteFile(&self, Path:&PathBuf, Content:Vec<u8>, Create:bool, Overwrite:bool) -> Result<(), CommonError>;

	/// Creates a directory at the specified path.
	///
	/// @param Path - The path of the directory to create.
	/// @param Recursive - If `true`, creates all parent directories as needed.
	async fn CreateDirectory(&self, Path:&PathBuf, Recursive:bool) -> Result<(), CommonError>;

	/// Deletes a file or directory.
	///
	/// @param Path - The path of the item to delete.
	/// @param Recursive - If `true`, deletes a directory and all its contents.
	/// @param UseTrash - If `true`, moves the item to the system's trash
	/// instead of   permanently deleting it.
	async fn Delete(&self, Path:&PathBuf, Recursive:bool, UseTrash:bool) -> Result<(), CommonError>;

	/// Renames (moves) a file or directory.
	///
	/// @param Source - The original path of the item.
	/// @param Target - The new path for the item.
	/// @param Overwrite - If `true`, an existing item at the target path will
	/// be overwritten.
	async fn Rename(&self, Source:&PathBuf, Target:&PathBuf, Overwrite:bool) -> Result<(), CommonError>;

	/// Copies a file or directory.
	///
	/// @param Source - The path of the item to copy.
	/// @param Target - The destination path for the copy.
	/// @param Overwrite - If `true`, an existing item at the target path will
	/// be overwritten.
	async fn Copy(&self, Source:&PathBuf, Target:&PathBuf, Overwrite:bool) -> Result<(), CommonError>;

	/// Creates a new, empty file at the specified path.
	async fn CreateFile(&self, Path:&PathBuf) -> Result<(), CommonError>;
}
