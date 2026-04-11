#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

//! # Common: Abstract Core Library for Code Editor Land
//!
//! Every Element in Land depends on Common. It defines the traits, services,
//! and data transfer objects that form the application's public API contract.
//! Mountain implements these traits. Wind and Cocoon consume them.
//!
//! ## Why Common Exists
//!
//! Common enforces a clean separation between "what the app can do" (traits)
//! and "how it does it" (Mountain's concrete implementations). This means:
//!
//! - You can test business logic without launching Tauri
//! - New backends can implement the same traits
//! - The TypeScript frontend only needs to know the DTO shapes
//!
//! ## Key Abstractions
//!
//! - **ActionEffect**: Declarative effect type executed by Mountain's runtime.
//!   Business logic is described as composable effects, not imperative calls.
//! - **Service traits**: FileSystemService, ProcessService, ExtensionService,
//!   and 15 more domain-specific contracts.
//! - **DTOs**: Type-safe data objects shared across IPC boundaries (gRPC, Tauri
//!   commands, WebSocket).
//! - **Error types**: Structured errors with context for every service domain.
//!
//! ## Module Layout
//!
//! | Module | Contents |
//! |---|---|
//! | `Effect/` | ActionEffect enum, builders, and combinators |
//! | `Environment/` | Capability provider trait (dependency injection) |
//! | `Error/` | Domain-specific error types with context |
//! | `Transport/` | Transport-agnostic communication (gRPC, IPC, WASM) |
//! | `DTO/` | Re-exported data transfer objects from all services |
//! | `Command/` .. `Workspace/` | Service trait definitions (20 domains) |
//!
//! ## Getting Started
//!
//! Common builds as part of the Land monorepo:
//! ```bash
//! cargo build -p Common
//! cargo test -p Common
//! ```
//!
//! Full setup: <https://github.com/CodeEditorLand/Land>

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

pub mod Webview;

pub mod Workspace;

// --- Transport Layer ---
// Provides transport-agnostic communication abstractions (gRPC, IPC, WASM)
pub mod Transport;

// --- Global DTO Module ---
//
// A top-level module that re-exports all Data Transfer Objects (DTOs) from the
// various service modules for convenient access across the application.
pub mod DTO;
