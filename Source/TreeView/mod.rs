//! # TreeView Service
//!
//! This module defines the abstract contract for the TreeView service. It
//! includes the `TreeViewProvider` trait, all related Data Transfer Objects
//! (DTOs), and will contain the `ActionEffect` constructors for every tree view
//! operation.

// --- Trait Definition ---
/// Trait for providing tree data to sidebar views.
pub mod TreeViewProvider;

// --- Data Transfer Objects ---
/// DTOs for the TreeView service.
pub mod DTO;
