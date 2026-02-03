// File: Common/Source/Webview/mod.rs
// Role: Public module interface for the Webview service contract.
// Responsibilities:
//   - Expose all necessary traits, DTOs, and effect constructors related to
//     Webviews.

//! # Webview Service
//!
//! This module defines the abstract contract for the Webview service. It
//! includes the `WebviewProvider` trait, all related Data Transfer Objects
//! (DTOs), and `ActionEffect` constructors for every Webview operation.

// --- Trait Definition ---
pub mod WebviewProvider;

// --- Data Transfer Objects ---
pub mod DTO;
