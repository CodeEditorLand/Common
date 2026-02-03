//! # CommonError Enum
//!
//! Defines the universal, structured error enum for the entire application
//! ecosystem.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A common error enum for the application, encompassing all major categories
/// of failures that can occur during the execution of effects or other
// operations.
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum CommonError {
	// --- FileSystem Errors ---
	#[error("FileSystem I/O error for '{Path}': {Description}")]
	FileSystemIO { Path:PathBuf, Description:String },

	#[error("Resource not found: {0}")]
	FileSystemNotFound(PathBuf),

	#[error("Permission denied for operation on '{Path}': {Reason}")]
	FileSystemPermissionDenied { Path:PathBuf, Reason:String },

	#[error("Resource already exists: {0}")]
	FileSystemFileExists(PathBuf),

	#[error("Path is not a directory: {0}")]
	FileSystemNotADirectory(PathBuf),

	#[error("Path is a directory (expected a file): {0}")]
	FileSystemIsADirectory(PathBuf),

	// --- Configuration Errors ---
	#[error("Configuration update error for key '{Key}': {Description}")]
	ConfigurationUpdate { Key:String, Description:String },

	#[error("Configuration load error: {Description}")]
	ConfigurationLoad { Description:String },

	// --- General Application Errors ---
	#[error("Invalid argument '{ArgumentName}': {Reason}")]
	InvalidArgument { ArgumentName:String, Reason:String },

	#[error("Feature not implemented: {FeatureName}")]
	NotImplemented { FeatureName:String },

	#[error("Internal state access error (e.g., lock poisoned): {Context}")]
	StateLockPoisoned { Context:String },

	#[error("Inter-process communication error: {Description}")]
	IPCError { Description:String },

	// --- Command System Errors ---
	#[error("Command '{CommandIdentifier}' execution failed: {Reason}")]
	CommandExecution { CommandIdentifier:String, Reason:String },

	#[error("Command '{Identifier}' not found")]
	CommandNotFound { Identifier:String },

	#[error("Feature '{FeatureName}' is not available")]
	FeatureNotAvailable { FeatureName:String },

	// --- Language Feature Provider Errors ---
	#[error("Language provider registration failed for '{ProviderType}': {Reason}")]
	ProviderRegistration { ProviderType:String, Reason:String },

	#[error("Language provider '{ProviderIdentifier}' invocation failed: {Reason}")]
	ProviderInvocation { ProviderIdentifier:String, Reason:String },

	// --- Test Framework Errors ---
	#[error("Test run '{RunIdentifier}' not found")]
	TestRunNotFound { RunIdentifier:String },

	#[error("Test controller '{ControllerIdentifier}' not found")]
	TestControllerNotFound { ControllerIdentifier:String },

	// --- TreeView Errors (New) ---
	#[error("TreeView provider not found for view ID '{ViewIdentifier}'")]
	TreeViewProviderNotFound { ViewIdentifier:String },

	#[error("WebView '{Handle}' not found")]
	WebViewNotFound { Handle:String },

	// --- Other Specific Errors ---
	#[error("External service '{ServiceName}' failed: {Description}")]
	ExternalServiceError { ServiceName:String, Description:String },

	#[error("Secret access for key '{Key}' failed: {Reason}")]
	SecretsAccess { Key:String, Reason:String },

	#[error("UserInterface interaction failed: {Reason}")]
	UserInterfaceInteraction { Reason:String },

	#[error("Serialization or Deserialization error: {Description}")]
	SerializationError { Description:String },

	// --- Catch-all Unknown Error ---
	#[error("An unknown internal error occurred: {Description}")]
	Unknown { Description:String },
}

impl CommonError {
	/// Creates a `CommonError` from a standard `std::io::Error`, mapping common
	/// I/O error kinds to our specific FileSystem error variants for better
	/// contextualization.
	pub fn FromStandardIOError(IOError:std::io::Error, Path:PathBuf, OperationContext:&str) -> Self {
		let Description = IOError.to_string();

		match IOError.kind() {
			std::io::ErrorKind::NotFound => CommonError::FileSystemNotFound(Path),

			std::io::ErrorKind::PermissionDenied => {
				CommonError::FileSystemPermissionDenied { Path, Reason:Description }
			},

			std::io::ErrorKind::AlreadyExists => CommonError::FileSystemFileExists(Path),

			_ => {
				CommonError::FileSystemIO {
					Path,

					Description:format!("Operation '{}' failed: {}", OperationContext, Description),
				}
			},
		}
	}
}

/// Converts a `serde_json::Error` into a `CommonError::SerializationError`.
impl From<serde_json::Error> for CommonError {
	fn from(SerdeError:serde_json::Error) -> Self {
		CommonError::SerializationError { Description:SerdeError.to_string() }
	}
}

/// Converts a `tauri::Error` into a `CommonError`. This is useful for
/// handling errors from Tauri's APIs within our effect system.
impl From<tauri::Error> for CommonError {
	fn from(TauriError:tauri::Error) -> Self { CommonError::UserInterfaceInteraction { Reason:TauriError.to_string() } }
}
