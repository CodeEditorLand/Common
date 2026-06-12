// File: Common/Source/Diagnostic/mod.rs
// Role: Public module interface for the Diagnostic service contract.
// Responsibilities:
//   - Expose all necessary traits and effect constructors related to
//     diagnostics.

//! # Diagnostic Service
//!
//! This module defines the abstract contract for the Diagnostic service, which
//! is responsible for managing problems like errors and warnings detected in
//! the workspace. It includes the `DiagnosticManager` trait and the
//! `ActionEffect` constructors for all diagnostic operations.

// --- Trait Definition ---
/// Trait for managing document diagnostics.
pub mod DiagnosticManager;

// --- Effect Constructors ---
/// Effect constructor for clearing all diagnostics.
pub mod ClearDiagnostics;

/// Effect constructor for retrieving all diagnostics.
pub mod GetAllDiagnostics;

/// Effect constructor for setting diagnostics on a document.
pub mod SetDiagnostics;
