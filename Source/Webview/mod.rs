// File: Common/Source/Webview/mod.rs
// Role: Public module interface for the Webview service contract.
// Responsibilities:
//   - Expose all necessary traits, DTOs, and effect constructors related to
//     Webviews.

//! # Webview Service
//!
//! Defines the abstract contract for the Webview service, including the
//! `WebviewProvider` trait, related Data Transfer Objects (DTOs), and
//! `ActionEffect` constructors for every Webview operation.

// --- Trait Definition ---
/// Trait for creating and managing webview instances.
pub mod WebviewProvider;

// --- Data Transfer Objects ---
/// DTOs for the Webview service.
pub mod DTO;
