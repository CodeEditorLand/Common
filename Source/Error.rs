// File: Common/Source/Error.rs
// Responsibility: Responsibility could not be determined.
// Modified: 2025-06-06 23:36:34 UTC

// Land_Common/src/errors.rs

use std::path::PathBuf;

use thiserror::Error;
// Serde is only needed if CommonError itself needs to be serialized across
// boundaries. For internal use within effects, it's often not required for the
// error enum itself. use serde::{Serialize, Deserialize};

/// A common error enum for the application, encompassing various error types
/// that can occur during the execution of effects or other operations.
#[derive(Error, Debug, Clone)] // Clone is useful for some error handling patterns
pub enum CommonError {
	// --- Filesystem Errors ---
	// These variants aim to be more specific than a generic FsError::Io(String)
	// by including the path and a more descriptive context.
	#[error("Filesystem I/O error for '{path}': {description}")]
	FsIo { path:PathBuf, description:String },

	#[error("Resource not found: {0}")]
	FsNotFound(PathBuf),

	#[error("Permission denied for operation on '{path}': {reason}")]
	FsPermissionDenied { path:PathBuf, reason:String },

	#[error("Resource already exists: {0}")]
	FsFileExists(PathBuf), // Or FsAlreadyExists if it can be a directory

	#[error("Path is not a directory: {0}")]
	FsNotADirectory(PathBuf),

	#[error("Path is a directory (expected a file): {0}")]
	FsIsADirectory(PathBuf),

	#[error("Directory not empty: {0}")]
	FsNotEmpty(PathBuf),

	#[error("Invalid path: {0}")]
	FsInvalidPath(PathBuf), // From FsError::InvalidPath

	#[error("Read error for '{path}': {description}")]
	FsRead { path:PathBuf, description:String },

	#[error("Write error for '{path}': {description}")]
	FsWrite { path:PathBuf, description:String },

	#[error("Stat error for '{path}': {description}")]
	FsStat { path:PathBuf, description:String },

	#[error("ReadDir error for '{path}': {description}")]
	FsReadDir { path:PathBuf, description:String },

	#[error("Mkdir error for '{path}': {description}")]
	FsMkdir { path:PathBuf, description:String },

	#[error("Delete error for '{path}': {description}")]
	FsDelete { path:PathBuf, description:String },

	#[error("Rename error from '{source}' to '{target}': {description}")]
	FsRename { source:PathBuf, target:PathBuf, description:String },

	#[error("Copy error from '{source}' to '{target}': {description}")]
	FsCopy { source:PathBuf, target:PathBuf, description:String },

	// --- Configuration Errors ---
	#[error("Configuration update error for '{key}': {description}")]
	ConfigUpdate { key:String, description:String },

	#[error("Configuration load error: {description}")]
	ConfigLoad { description:String },

	// --- General Application Errors ---
	#[error("Invalid argument '{argument_name}': {reason}")]
	InvalidArg { argument_name:String, reason:String },

	#[error("Feature not implemented: {feature_name}")]
	NotImplemented { feature_name:String },

	#[error("Internal state access error (e.g., lock poisoned): {context}")]
	StateLock { context:String },

	#[error("Inter-process communication error: {description}")]
	IpcError { description:String }, // Can wrap errors from IPC mechanisms like Vine

	// --- Command System Errors ---
	#[error("Command '{command_id}' execution failed: {reason}")]
	CommandExecution { command_id:String, reason:String },

	#[error("Command '{command_id}' registration failed: {reason}")]
	CommandRegistration { command_id:String, reason:String },

	#[error("Failed to list commands: {reason}")]
	CommandList { reason:String },

	// --- Language Feature Provider Errors ---
	#[error("Language provider registration failed for '{provider_type}': {reason}")]
	ProviderRegistration {
		provider_type:String, // Consider using a ProviderType enum if available
		reason:String,
	},

	#[error("Language provider '{provider_id}' invocation failed: {reason}")]
	ProviderInvocation {
		provider_id:String, // Or handle/type
		reason:String,
	},

	#[error("No provider found for feature '{feature}' on document '{document_uri}'")]
	ProviderNotFound {
		feature:String,
		document_uri:String, // Or Url
	},

	// --- Other Specific Errors ---
	#[error("Secret access for key '{key}' failed: {reason}")]
	SecretsAccess { key:String, reason:String },

	#[error("Output channel '{channel_name}' operation failed: {reason}")]
	OutputChannel { channel_name:String, reason:String },

	#[error("Diagnostics operation failed: {reason}")]
	Diagnostics { reason:String },

	#[error("UI interaction failed: {reason}")]
	UiInteraction { reason:String }, // Used by UiProvider effects

	#[error("Serialization or Deserialization error: {description}")]
	SerdeError { description:String },

	// --- Catch-all Unknown Error ---
	#[error("An unknown internal error occurred: {description}")]
	Unknown { description:String },
}

// Helper for converting std::io::Error to CommonError for filesystem
// operations. This provides a basic mapping. Implementations of
// FsReader/FsWriter might want to do more specific mapping based on the
// operation.
impl CommonError {
	pub fn from_std_io_error(err:std::io::Error, path:PathBuf, operation_context:&str) -> Self {
		let description = err.to_string();
		match err.kind() {
			std::io::ErrorKind::NotFound => CommonError::FsNotFound(path),
			std::io::ErrorKind::PermissionDenied => CommonError::FsPermissionDenied { path, reason:description },
			std::io::ErrorKind::AlreadyExists => CommonError::FsFileExists(path),
			// Add more specific mappings if needed
			_ => CommonError::FsIo { path, description:format!("{} failed: {}", operation_context, description) },
		}
	}
}

// If serde errors need to be wrapped:
impl From<serde_json::Error> for CommonError {
	fn from(err:serde_json::Error) -> Self { CommonError::SerdeError { description:err.to_string() } }
}

// The DirEntryInfo struct is a DTO, not an error. It should live in a module
// related to filesystem data structures if it's used by FsReader/FsWriter
// traits, or alongside effects that produce/consume it. It was present in the
// FsError snippets but doesn't belong in `errors.rs`.
//
// Example:
// // 
// #[derive(Debug, Serialize, Deserialize)]
// pub struct DirEntryInfo {
//     pub name: String,
//     pub is_dir: bool,
//     // pub file_type: FileType, // Using the FileType enum is more robust
//     // pub size: Option<u64>,
//     // pub mtime: Option<u64>,
// }
