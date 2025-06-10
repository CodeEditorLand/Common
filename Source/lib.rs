

/**
 * @module Common Crate
 * @description This crate defines the abstract architectural core for the entire
 * application ecosystem. It provides a declarative, effects-based system for
 * building application logic, ensuring a clean separation between the definition
 * of an operation (`ActionEffect`) and its concrete implementation (`Environment`).
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Core Architecture ---
pub mod effect;
pub mod environment;
pub mod error;

// --- Service Contracts (alphabetical) ---
pub mod command;
pub mod config;
pub mod custom_editor;
pub mod diagnostics;
pub mod document;
pub mod fs;
pub mod ipc;
pub mod language_feature;
pub mod output;
pub mod scm;
pub mod secrets;
pub mod status_bar;
pub mod storage;
pub mod sync;
pub mod terminal;
pub mod testing;
pub mod tree_view;
pub mod ui;
pub mod webview;
pub mod workspace;

// --- Global DTO Module ---
/**
 * A top-level module that re-exports all Data Transfer Objects (DTOs) from the
 * various service modules for convenient access.
 */
pub mod dto {
    pub use crate::config::dto::*;
    pub use crate::fs::dto::*;
    pub use crate::ipc::dto::*;
    pub use crate::language_feature::dto::*;
    pub use crate::scm::dto::*;
    pub use crate::status_bar::dto::*;
    pub use crate::tree_view::dto::*;
    pub use crate::ui::dto::*;
    pub use crate::webview::dto::*;
    // Add other DTO re-exports as they are created
}
