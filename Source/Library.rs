// File: Common/Library.rs
// Serves as the main library entry point for the `Common` crate,
// declaring and re-exporting all core effects, DTOs, and traits.

#![allow(non_snake_case, non_camel_case_types)]

// Core Modules
pub mod Effect;
pub mod Environment;
pub mod Errors;
pub mod Runtime;

// Effect and DTO Modules (Grouped by Feature)
pub mod CommandEffect;
pub mod ConfigEffect;
pub mod ConfigurationDto; // DTOs for ConfigEffect
pub mod DiagnosticsEffect;
pub mod DocumentDto; // DTOs for DocumentEffect
pub mod DocumentEffect;
pub mod FileSystemDto; // DTOs for FsEffect
pub mod FsEffect;
pub mod HasEnvironment; // A utility trait
pub mod IpcDto; // DTOs for IpcEffect
pub mod IpcEffect;
pub mod LanguageFeatureDto; // DTOs for LanguageFeatureEffect
pub mod LanguageFeatureEffect;
pub mod OutputEffect;
pub mod SecretsEffect;
pub mod StorageEffect;
pub mod UiDto; // DTOs for UiEffect
pub mod UiEffect;
pub mod WorkspaceEffect;

// Re-exporting DTO modules for convenient access under a consistent namespace.
// This was part of the original design and is kept for compatibility.
// A more modern approach might have each `...Effect` module export its own
// DTOs directly. This structure is maintained as per the provided file
// contents.
pub mod Dto {
	pub mod Configuration {
		pub use crate::ConfigurationDto::*;
	}
	pub mod Document {
		pub use crate::DocumentDto::*;
	}
	pub mod FileSystem {
		pub use crate::FileSystemDto::*;
	}
	pub mod LanguageFeature {
		pub use crate::LanguageFeatureDto::*;
	}
	pub mod Ipc {
		pub use crate::IpcDto::*;
	}
	pub mod Ui {
		pub use crate::UiDto::*;
	}
}
