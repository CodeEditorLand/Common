// File: Common/Source/SourceControlManagement/mod.rs
// Role: Public module interface for the Source Control Management
// (SourceControlManagement) service contract. Responsibilities:
//   - Expose all necessary traits, DTOs, and effect constructors related to
//     SourceControlManagement.

//! # SourceControlManagement Service
//!
//! This module defines the abstract contract for the Source Control Management
//! (SourceControlManagement) service, which is responsible for integrating with
//! version control systems like Git.

// --- Trait Definition ---
/// Trait for source control management (VCS integration).
pub mod SourceControlManagementProvider;

// --- Data Transfer Objects ---
/// DTOs for the Source Control Management API.
pub mod DTO;
