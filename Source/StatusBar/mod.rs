// File: Common/Source/StatusBar/mod.rs
// Role: Public module interface for the StatusBar service contract.
// Responsibilities:
//   - Expose all necessary traits, DTOs, and effect constructors related to the
//     status bar.

//! # StatusBar Service
//!
//! Defines the abstract contract for the StatusBar service, including the
//! `StatusBarProvider` trait, related Data Transfer Objects (DTOs), and
//! `ActionEffect` constructors for status bar operations.

// --- Trait Definition ---
/// Trait for managing status bar items and messages.
pub mod StatusBarProvider;

// --- Data Transfer Objects ---
/// DTOs for the StatusBar service.
pub mod DTO;
