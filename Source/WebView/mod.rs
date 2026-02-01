// File: Common/Source/WebView/mod.rs
// Role: Public module interface for the WebView service contract.
// Responsibilities:
//   - Expose all necessary traits, DTOs, and effect constructors related to
//     WebViews.

//! # WebView Service
//!
//! This module defines the abstract contract for the WebView service. It
//! includes the `WebViewProvider` trait, all related Data Transfer Objects
//! (DTOs), and `ActionEffect` constructors for every WebView operation.

// --- Trait Definition ---
pub mod WebViewProvider;

// --- Data Transfer Objects ---
pub mod DTO;
