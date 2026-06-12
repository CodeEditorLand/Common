//! # Global DTO Module
//!
//! A top-level module that re-exports all Data Transfer Objects (DTOs) from the
//! various service modules for convenient access across the application.
//! It also contains DTOs that are shared across multiple services.

/// DTO for an edit to a workspace (text edits, file operations).
pub mod WorkspaceEditDTO;
