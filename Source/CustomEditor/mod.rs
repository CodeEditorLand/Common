// File: Common/Source/CustomEditor/mod.rs
// Role: Public module interface for the Custom Editor service contract.
// Responsibilities:
//   - Expose all necessary traits, DTOs, and effect constructors related to
//     custom editors.

//! # CustomEditor Service
//!
//! This module defines the abstract contract for the Custom Editor service,
//! which allows extensions to provide custom, WebView-based editors for
//! specific file types.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod CustomEditorProvider;
// pub use self::CustomEditorProvider::CustomEditorProvider;

// --- Data Transfer Objects ---
// pub mod DTO;

// --- Effect Constructors ---
// pub mod OnSaveCustomDocument;
// pub mod RegisterCustomEditor;
// pub mod ResolveCustomEditor;
// pub mod UnregisterCustomEditor;

// pub use self::{
// 	OnSaveCustomDocument::OnSaveCustomDocument,
// 	RegisterCustomEditor::RegisterCustomEditor,
// 	ResolveCustomEditor::ResolveCustomEditor,
// 	UnregisterCustomEditor::UnregisterCustomEditor,
// };
