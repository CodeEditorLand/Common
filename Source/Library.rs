#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

//! # Common Crate
//!
//! Defines the abstract architectural core for the entire application
//! ecosystem. It provides the foundational traits, services, and data transfer
//! objects (DTOs) that constitute the application's public API contract.
//!
//! This crate enforces a clean separation of concerns by defining "what" an
//! operation does (the service trait, e.g., `FileSystemReader`) separately
//! from "how" it is implemented (the concrete implementation in the `Mountain`
//! crate). This declarative, effects-based architecture ensures that
//! application logic is composable, testable, and maintainable.

// --- Core Architecture ---
pub mod Effect;

pub mod Environment;

pub mod Error;

pub mod Utility;

// --- Service Contracts (alphabetical) ---
pub mod Command;

pub mod Configuration;

pub mod CustomEditor;

pub mod Debug;

pub mod Diagnostic;

pub mod Document;

pub mod ExtensionManagement;

pub mod FileSystem;

pub mod IPC;

pub mod Keybinding;

pub mod LanguageFeature;

pub mod Output;

pub mod Search;

pub mod Secret;

pub mod SourceControlManagement;

pub mod StatusBar;

pub mod Storage;

pub mod Synchronization;

pub mod Terminal;

pub mod Testing;

pub mod TreeView;

pub mod UserInterface;

pub mod WebView;

pub mod WorkSpace;

// --- Global DTO Module ---
//
// A top-level module that re-exports all Data Transfer Objects (DTOs) from the
// various service modules for convenient access across the application.
pub mod DTO;
