//! # TreeView Service
//!
//! Defines the abstract contract for the TreeView service, including the
//! `TreeViewProvider` trait, related Data Transfer Objects (DTOs), and
//! `ActionEffect` constructors for every tree view operation.

// --- Trait Definition ---
/// Trait for providing tree data to sidebar views.
pub mod TreeViewProvider;

// --- Data Transfer Objects ---
/// DTOs for the TreeView service.
pub mod DTO;
