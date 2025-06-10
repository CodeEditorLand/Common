/// @module CommonError
/// @description Defines the universal, structured error enum for the
/// application.
use std::path::PathBuf;

use thiserror::Error;

/// A common error enum for the application, encompassing all major categories
/// of failures that can occur during the execution of effects or other
/// operations. Using a single, comprehensive error enum allows for precise,
/// exhaustive pattern matching in error handling logic.
#[derive(Error, Debug, Clone)]
pub enum CommonError {
	// --- Filesystem Errors ---
	#[error("Filesystem I/O error for '{Path}': {Description}")]
	FsIo { Path:PathBuf, Description:String },
	#[error("Resource not found: {0}")]
	FsNotFound(PathBuf),
	#[error("Permission denied for operation on '{Path}': {Reason}")]
	FsPermissionDenied { Path:PathBuf, Reason:String },
	#[error("Resource already exists: {0}")]
	FsFileExists(PathBuf),
	#[error("Path is not a directory: {0}")]
	FsNotADirectory(PathBuf),
	#[error("Path is a directory (expected a file): {0}")]
	FsIsADirectory(PathBuf),

	// --- Configuration Errors ---
	#[error("Configuration update error for key '{Key}': {Description}")]
	ConfigUpdate { Key:String, Description:String },
	#[error("Configuration load error: {Description}")]
	ConfigLoad { Description:String },

	// --- General Application Errors ---
	#[error("Invalid argument '{ArgumentName}': {Reason}")]
	InvalidArg { ArgumentName:String, Reason:String },
	#[error("Feature not implemented: {FeatureName}")]
	NotImplemented { FeatureName:String },
	#[error("Internal state access error (e.g., lock poisoned): {Context}")]
	StateLock { Context:String },
	#[error("Inter-process communication error: {Description}")]
	IpcError { Description:String },

	// --- Command System Errors ---
	#[error("Command '{CommandIdentifier}' execution failed: {Reason}")]
	CommandExecution { CommandIdentifier:String, Reason:String },
	#[error("Command '{CommandIdentifier}' not found")]
	CommandNotFound { Feature:String, DocumentUri:String },

	// --- Language Feature Provider Errors ---
	#[error("Language provider registration failed for '{ProviderType}': {Reason}")]
	ProviderRegistration { ProviderType:String, Reason:String },
	#[error("Language provider '{ProviderIdentifier}' invocation failed: {Reason}")]
	ProviderInvocation { ProviderIdentifier:String, Reason:String },

	// --- Other Specific Errors ---
	#[error("Secret access for key '{Key}' failed: {Reason}")]
	SecretsAccess { Key:String, Reason:String },
	#[error("UI interaction failed: {Reason}")]
	UiInteraction { Reason:String },
	#[error("Serialization or Deserialization error: {Description}")]
	SerdeError { Description:String },

	// --- Catch-all Unknown Error ---
	#[error("An unknown internal error occurred: {Description}")]
	Unknown { Description:String },
}

impl CommonError {
	/// Creates a `CommonError` from a standard `std::io::Error`, mapping common
	/// I/O error kinds to our specific filesystem error variants.
	pub fn FromStdIoError(StdIoError:std::io::Error, Path:PathBuf, OperationContext:&str) -> Self {
		let Description = StdIoError.to_string();
		match StdIoError.kind() {
			std::io::ErrorKind::NotFound => CommonError::FsNotFound(Path),
			std::io::ErrorKind::PermissionDenied => CommonError::FsPermissionDenied { Path, Reason:Description },
			std::io::ErrorKind::AlreadyExists => CommonError::FsFileExists(Path),
			_ => {
				CommonError::FsIo {
					Path,
					Description:format!("Operation '{}' failed: {}", OperationContext, Description),
				}
			},
		}
	}
}

/// Converts a `serde_json::Error` into a `CommonError::SerdeError`.
/// This enables the use of the `?` operator on `serde_json` results within
/// functions that return a `Result<_, CommonError>`.
impl From<serde_json::Error> for CommonError {
	fn from(SerdeError:serde_json::Error) -> Self { CommonError::SerdeError { Description:SerdeError.to_string() } }
}
