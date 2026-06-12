#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(
	non_snake_case,
	non_camel_case_types,
	non_upper_case_globals,
	dead_code,
	unused_imports,
	unused_variables,
	unused_assignments
)]

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
//! ## Architecture
//!
//! For a complete overview of the system architecture, process model,
//! IPC design, service layers, and data flow patterns, see
//! [Architecture.md](https://github.com/CodeEditorLand/Land/blob/main/Documentation/GitHub/Architecture.md)
//! (`Documentation/GitHub/Architecture.md` in the checkout).
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

/// Declarative effect types and runtime for composing async operations.
pub mod Effect;

/// Capability-provider trait (dependency injection root).
pub mod Environment;

/// Domain-specific error types (`CommonError`) with structured context.
pub mod Error;

/// Shared serialization helpers used across the crate.
pub mod Utility;

// --- Service Contracts (alphabetical) ---

/// Command registration, execution, and lifecycle management service.
pub mod Command;

/// Application configuration (get, inspect, update) service contract.
pub mod Configuration;

/// Custom editor provider for domain-specific editor surfaces.
pub mod CustomEditor;

/// Debug service contract (sessions, breakpoints, DAP messages).
pub mod Debug;

/// Diagnostic manager for tracking and clearing document diagnostics.
pub mod Diagnostic;

/// Document lifecycle service (open, save, apply changes).
pub mod Document;

/// Extension management service (install, uninstall, query extensions).
pub mod ExtensionManagement;

/// File system read, write, watch, and directory operations.
pub mod FileSystem;

/// Inter-process communication channel registry, events, and provider trait.
pub mod IPC;

/// Keybinding resolution and management service.
pub mod Keybinding;

/// Language feature provider registry (completions, hover, go-to-def, etc.).
pub mod LanguageFeature;

/// Output channel management for logging and display surfaces.
pub mod Output;

/// File and text search across workspace files.
pub mod Search;

/// Secret storage (get, store, delete) service contract.
pub mod Secret;

/// Source control management provider contract.
pub mod SourceControlManagement;

/// Status bar entry provider for custom status bar items.
pub mod StatusBar;

/// Persistent key-value storage service contract.
pub mod Storage;

/// Workspace synchronization provider contract.
pub mod Synchronization;

/// Terminal creation and lifecycle management service.
pub mod Terminal;

/// Test controller provider for running and managing tests.
pub mod Testing;

/// Tree view provider for sidebar tree data sources.
pub mod TreeView;

/// User interface dialogs, messages, quick pick, and input box.
pub mod UserInterface;

/// Webview creation and lifecycle management service.
pub mod Webview;

/// Workspace provider (folders, edits, trust, file opening).
pub mod Workspace;

// --- Transport Layer ---
/// Transport-agnostic communication abstractions (gRPC, IPC, WASM).
pub mod Transport;

// --- Telemetry ---
/// Shared dual-pipe (PostHog + OTLP) emit module. Used by every Rust
/// sidecar (Air, Echo, Rest, Grove, Mist, SideCar). Mountain keeps its
/// own compile-time-baked plugin under `Binary/Build/PostHogPlugin/*`.
pub mod Telemetry;

// --- Global DTO Module ---
/// A top-level module that re-exports all Data Transfer Objects (DTOs) from the
/// various service modules for convenient access across the application.
pub mod DTO;
