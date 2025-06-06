// ---------------------------------------------------------------------------------------------
// Filesystem Effect (fs_effects.rs)
// ---------------------------------------------------------------------------------------------
// Defines traits (`FsReader`, `FsWriter`) for abstracting filesystem operations
// and provides `ActionEffect` constructors for common FS tasks.
// These effects declare dependencies on the FS traits, which are then provided
// by a concrete `Environment` implementation, accessible via an
// `AppRuntime`-like context.
// ---------------------------------------------------------------------------------------------

use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// serde_json::Value might be used if some API explicitly requires returning JSON,
// but we'll prefer specific DTOs/types where possible.
// use serde_json::{json, Value};
use crate::{
	effect::ActionEffect,
	// Environment and Requires are fundamental to how effects get their dependencies.
	// AppRuntime is the typical context passed to an effect's closure.
	environment::{Environment, Requires},
	errors::CommonError,
	// Assume an AppRuntime or similar accessor is defined, potentially with a helper trait like HasEnvironment
	// from previous examples. For fs_effects, they would depend on FsReader/FsWriter.
	// The generic type `AR` in effects will represent this AppRuntime.
};

// --- Filesystem DTOs ---

/// Represents the type of a file system entry.
/// Values align with VS Code's internal `FileType` enum for interoperability.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FileType {
	Unknown = 0,
	File = 1,
	Directory = 2,
	SymbolicLink = 64, // VS Code: FileType.SymbolicLink = 64
}

/// Represents metadata for a file or directory, similar to VS Code's
/// `FileStat`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileSystemStat {
	/// Type of the file (File, Directory, SymbolicLink, or Unknown).
	/// Encoded as u8 for direct use of `FileType` enum values.
	#[serde(rename = "type")]
	pub file_type:u8,
	/// Creation time in milliseconds since epoch.
	pub ctime:u64,
	/// Modification time in milliseconds since epoch.
	pub mtime:u64,
	/// Size in bytes.
	pub size:u64,
	/// File permissions. Optional, aligns with vscode.FilePermission (e.g.,
	/// Readonly = 1).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub permissions:Option<u32>,
}

// --- Filesystem Dependency Traits ---

/// Trait abstracting read-only filesystem operations.
/// Implementors are expected to be `Send + Sync`.
/// The `Environment` bound ensures it can be part of the dependency injection
/// system.
#[async_trait]
pub trait FsReader: Environment + Send + Sync {
	/// Reads the entire content of a file into a byte vector.
	/// Returns `CommonError::FsNotFound` if the path does not exist or is not a
	/// file.
	async fn read_file(&self, path:&PathBuf) -> Result<Vec<u8>, CommonError>;

	/// Reads metadata for a file or directory.
	/// Returns `CommonError::FsNotFound` if the path does not exist.
	async fn stat_file(&self, path:&PathBuf) -> Result<FileSystemStat, CommonError>;

	/// Reads directory entries, returning a list of (name, FileType) tuples.
	/// Returns `CommonError::FsNotFound` if the path does not exist.
	/// Returns `CommonError::FsNotADirectory` if the path is not a directory.
	async fn read_directory(&self, path:&PathBuf) -> Result<Vec<(String, FileType)>, CommonError>;
}

/// Trait abstracting write/modifying filesystem operations.
/// Implementors are expected to be `Send + Sync`.
/// The `Environment` bound ensures it can be part of the dependency injection
/// system.
#[async_trait]
pub trait FsWriter: Environment + Send + Sync {
	/// Writes byte content to a file.
	///
	/// # Arguments
	/// * `path`: The path to the file.
	/// * `content`: The byte content to write.
	/// * `create`: If true, create the file if it does not exist. Parent
	///   directories should also be created.
	/// * `overwrite`: If true, overwrite the file if it exists. If false and
	///   file exists, an error should be returned.
	async fn write_file(&self, path:&PathBuf, content:Vec<u8>, create:bool, overwrite:bool) -> Result<(), CommonError>;

	/// Creates a directory.
	///
	/// # Arguments
	/// * `path`: The path to the directory.
	/// * `recursive`: If true, create parent directories if they do not exist.
	/// If false and parents don't exist, an error should be returned.
	/// Should succeed if the directory already exists.
	async fn create_directory(&self, path:&PathBuf, recursive:bool) -> Result<(), CommonError>;

	/// Deletes a file or directory.
	///
	/// # Arguments
	/// * `path`: The path to delete.
	/// * `recursive`: If true and `path` is a directory, delete recursively. If
	///   false and `path` is a non-empty directory, an error should be
	///   returned.
	/// * `use_trash`: If true, attempt to move to system trash/recycle bin
	///   instead of permanent deletion. Behavior may vary by OS and
	///   implementation.
	/// Should succeed if the path does not exist (idempotent).
	async fn delete(&self, path:&PathBuf, recursive:bool, use_trash:bool) -> Result<(), CommonError>;

	/// Renames (moves) a file or directory.
	///
	/// # Arguments
	/// * `source`: The current path of the file/directory.
	/// * `target`: The new path for the file/directory.
	/// * `overwrite`: If true, replace the target if it exists. If false and
	///   the target exists, an error should be returned.
	async fn rename(&self, source:&PathBuf, target:&PathBuf, overwrite:bool) -> Result<(), CommonError>;

	/// Copies a file or directory.
	///
	/// # Arguments
	/// * `source`: The path of the file/directory to copy.
	/// * `target`: The path to copy to.
	/// * `overwrite`: If true, replace the target if it exists. If false and
	///   the target exists, an error should be returned.
	/// Note: Recursive directory copy needs careful implementation.
	async fn copy(&self, source:&PathBuf, target:&PathBuf, overwrite:bool) -> Result<(), CommonError>;

	/// Creates an empty file.
	/// This is a convenience method. Implementations might use `write_file`
	/// with empty content. Should fail if the file already exists, unless
	/// `write_file` with `overwrite=false` handles this. Parent directories
	/// should be created if they don't exist.
	async fn create_file(&self, path:&PathBuf) -> Result<(), CommonError>;
}

// Helper trait for ActionEffect context, assuming AppRuntime provides access to
// Environment This is similar to the HasEnvironment trait from
// language_feature_effects.rs This ensures that the AppRuntime (`AR`) can
// provide the necessary FsReader/FsWriter.
pub trait HasFsAccessors {
	type Env: Environment + Requires<Arc<dyn FsReader>> + Requires<Arc<dyn FsWriter>> + Send + Sync;
	fn get_environment(&self) -> Arc<Self::Env>;
}

// --- Filesystem ActionEffect Constructors ---

/// Effect to read the content of a file.
/// Returns the content as a `Vec<u8>`.
pub fn read_file<AR>(path:PathBuf) -> ActionEffect<Arc<AR>, CommonError, Vec<u8>>
where
	AR: HasFsAccessors + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |app_runtime_accessor| {
		let p_clone = path.clone();
		Box::pin(async move {
			let env = app_runtime_accessor.get_environment();
			let fs_reader:Arc<dyn FsReader> = env.require();
			fs_reader.read_file(&p_clone).await
		})
	}))
}

/// Effect to get file metadata (stat).
/// Returns metadata as a `FileSystemStat` struct.
pub fn stat_file<AR>(path:PathBuf) -> ActionEffect<Arc<AR>, CommonError, FileSystemStat>
where
	AR: HasFsAccessors + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |app_runtime_accessor| {
		let p_clone = path.clone();
		Box::pin(async move {
			let env = app_runtime_accessor.get_environment();
			let fs_reader:Arc<dyn FsReader> = env.require();
			fs_reader.stat_file(&p_clone).await
		})
	}))
}

/// Effect to read directory entries.
/// Returns a `Vec` of `(String, FileType)` tuples.
pub fn read_directory<AR>(path:PathBuf) -> ActionEffect<Arc<AR>, CommonError, Vec<(String, FileType)>>
where
	AR: HasFsAccessors + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |app_runtime_accessor| {
		let p_clone = path.clone();
		Box::pin(async move {
			let env = app_runtime_accessor.get_environment();
			let fs_reader:Arc<dyn FsReader> = env.require();
			fs_reader.read_directory(&p_clone).await
		})
	}))
}

/// Effect to write byte content to a file.
pub fn write_file_bytes<AR>(
	path:PathBuf,
	content:Vec<u8>,
	create:bool,
	overwrite:bool,
) -> ActionEffect<Arc<AR>, CommonError, ()>
where
	AR: HasFsAccessors + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |app_runtime_accessor| {
		let p_clone = path.clone();
		let c_clone = content.clone();
		Box::pin(async move {
			let env = app_runtime_accessor.get_environment();
			let fs_writer:Arc<dyn FsWriter> = env.require();
			fs_writer.write_file(&p_clone, c_clone, create, overwrite).await
		})
	}))
}

/// Convenience effect to write string content to a file.
pub fn write_file_string<AR>(
	path:PathBuf,
	content:String,
	create:bool,
	overwrite:bool,
) -> ActionEffect<Arc<AR>, CommonError, ()>
where
	AR: HasFsAccessors + Send + Sync + 'static, {
	write_file_bytes(path, content.into_bytes(), create, overwrite)
}

/// Effect to create a directory.
pub fn create_directory<AR>(path:PathBuf, recursive:bool) -> ActionEffect<Arc<AR>, CommonError, ()>
where
	AR: HasFsAccessors + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |app_runtime_accessor| {
		let p_clone = path.clone();
		Box::pin(async move {
			let env = app_runtime_accessor.get_environment();
			let fs_writer:Arc<dyn FsWriter> = env.require();
			fs_writer.create_directory(&p_clone, recursive).await
		})
	}))
}

/// Effect to create an empty file.
/// This uses `FsWriter::create_file` method.
pub fn create_file<AR>(path:PathBuf) -> ActionEffect<Arc<AR>, CommonError, ()>
where
	AR: HasFsAccessors + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |app_runtime_accessor| {
		let p_clone = path.clone();
		Box::pin(async move {
			let env = app_runtime_accessor.get_environment();
			let fs_writer:Arc<dyn FsWriter> = env.require();
			fs_writer.create_file(&p_clone).await
		})
	}))
}
// // Alternative implementation for create_file using write_file_bytes:
// pub fn create_file<AR>(path: PathBuf) -> ActionEffect<Arc<AR>, CommonError,
// ()> where
//     AR: HasFsAccessors + Send + Sync + 'static,
// {
//     // Create, do not overwrite if it exists (write_file handles
// overwrite=false logic)     write_file_bytes(path, Vec::new(), true, false)
// }

/// Effect to delete a file or directory.
pub fn delete<AR>(path:PathBuf, recursive:bool, use_trash:bool) -> ActionEffect<Arc<AR>, CommonError, ()>
where
	AR: HasFsAccessors + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |app_runtime_accessor| {
		let p_clone = path.clone();
		Box::pin(async move {
			let env = app_runtime_accessor.get_environment();
			let fs_writer:Arc<dyn FsWriter> = env.require();
			fs_writer.delete(&p_clone, recursive, use_trash).await
		})
	}))
}

/// Effect to rename (move) a file or directory.
pub fn rename<AR>(source:PathBuf, target:PathBuf, overwrite:bool) -> ActionEffect<Arc<AR>, CommonError, ()>
where
	AR: HasFsAccessors + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |app_runtime_accessor| {
		let s_clone = source.clone();
		let t_clone = target.clone();
		Box::pin(async move {
			let env = app_runtime_accessor.get_environment();
			let fs_writer:Arc<dyn FsWriter> = env.require();
			fs_writer.rename(&s_clone, &t_clone, overwrite).await
		})
	}))
}

/// Effect to copy a file or directory.
pub fn copy<AR>(source:PathBuf, target:PathBuf, overwrite:bool) -> ActionEffect<Arc<AR>, CommonError, ()>
where
	AR: HasFsAccessors + Send + Sync + 'static, {
	ActionEffect::new(Arc::new(move |app_runtime_accessor| {
		let s_clone = source.clone();
		let t_clone = target.clone();
		Box::pin(async move {
			let env = app_runtime_accessor.get_environment();
			let fs_writer:Arc<dyn FsWriter> = env.require();
			fs_writer.copy(&s_clone, &t_clone, overwrite).await
		})
	}))
}

// Note on Environment and Requires traits (expected to be in
// crate::environment):
//
// pub trait Environment: Send + Sync {} // Marker trait
// pub trait Requires<T>: Environment { fn require(&self) -> T; }
//
// The `AppRuntime` (represented by `AR` generic) is expected to have a method
// like `get_environment()` which returns an `Arc<Env>`, where `Env` implements
// `Requires<Arc<dyn FsReader>>` and `Requires<Arc<dyn FsWriter>>`.
// The `HasFsAccessors` trait formalizes this expectation for `AR`.
//
// Example (to be placed in the environment's module for FsReader/FsWriter):
//
//   use crate::common::fs_effects::{FsReader, FsWriter};
//   use crate::environment::Requires;
//   use std::sync::Arc;
//
//   impl Requires<Arc<dyn FsReader>> for MyEnvironment {
//       fn require(&self) -> Arc<dyn FsReader> {
//           // Return the actual FsReader implementation
//           self.my_fs_service.clone() // Assuming MyFsService implements
// FsReader       }
//   }
//   impl Requires<Arc<dyn FsWriter>> for MyEnvironment {
//       fn require(&self) -> Arc<dyn FsWriter> {
//           // Return the actual FsWriter implementation
//           self.my_fs_service.clone() // Assuming MyFsService implements
// FsWriter       }
//   }
