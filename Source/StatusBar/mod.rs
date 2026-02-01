// File: Common/Source/StatusBar/mod.rs
// Role: Public module interface for the StatusBar service contract.
// Responsibilities:
//   - Expose all necessary traits, DTOs, and effect constructors related to the
//     status bar.

//! # StatusBar Service
//!
//! This module defines the abstract contract for the StatusBar service. It
//! includes the `StatusBarProvider` trait, all related Data Transfer Objects
//! (DTOs), and `ActionEffect` constructors for status bar operations.

// --- Trait Definition ---
pub mod StatusBarProvider;

// --- Data Transfer Objects ---
pub mod DTO;
