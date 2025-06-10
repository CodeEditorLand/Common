

/**
 * @module custom_editor
 * @description This module defines the abstract contract for the Custom Editor
 * service, which allows extensions to provide custom, webview-based editors
 * for specific file types.
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod CustomEditorProvider;
pub use self::CustomEditorProvider::CustomEditorProvider;

// --- Data Transfer Objects ---
pub mod dto;

// --- Effect Constructors ---
mod OnSaveCustomDocument;
mod RegisterCustomEditorProvider;
mod ResolveCustomEditor;
mod UnregisterCustomEditorProvider;

pub use self::OnSaveCustomDocument::OnSaveCustomDocument;
pub use self::RegisterCustomEditorProvider::RegisterCustomEditorProvider;
pub use self::ResolveCustomEditor::ResolveCustomEditor;
pub use self::UnregisterCustomEditorProvider::UnregisterCustomEditorProvider;
