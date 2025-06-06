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
pub mod CommandEffects;
pub mod ConfigEffects;
pub mod ConfigurationDto; // DTOs for ConfigEffects
pub mod DiagnosticsEffects;
pub mod DocumentDto; // DTOs for DocumentEffects
pub mod DocumentEffects;
pub mod FileSystemDto; // DTOs for FsEffects
pub mod FsEffects;
pub mod HasEnvironment; // A utility trait
pub mod IpcDto; // DTOs for IpcEffects
pub mod IpcEffects;
pub mod LanguageFeatureDto; // DTOs for LanguageFeatureEffects
pub mod LanguageFeatureEffects;
pub mod OutputEffects;
pub mod SecretsEffects;
pub mod StorageEffects;
pub mod UiDto; // DTOs for UiEffects
pub mod UiEffects;
pub mod WorkspaceEffects;

// Re-exporting DTO modules for convenient access under a consistent namespace.
// This was part of the original design and is kept for compatibility.
// A more modern approach might have each `...Effects` module export its own
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
