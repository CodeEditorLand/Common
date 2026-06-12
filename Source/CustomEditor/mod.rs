// File: Common/Source/CustomEditor/mod.rs
// Role: Public module interface for the Custom Editor service contract.
// Responsibilities:
//   - Expose all necessary traits, DTOs, and effect constructors related to
//     custom editors.

//! # CustomEditor Service
//!
//! Defines the abstract contract for the Custom Editor service, which allows
//! extensions to provide custom, Webview-based editors for specific file
//! types.

// --- Trait Definition ---
/// Trait for providing custom editor implementations for specific file types.
pub mod CustomEditorProvider;
