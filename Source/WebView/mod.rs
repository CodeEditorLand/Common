//! # WebView Service
//!
//! This module defines the abstract contract for the WebView service. It
//! includes the `WebViewProvider` trait, all related Data Transfer Objects
//! (DTOs), and will contain `ActionEffect` constructors for every WebView
//! operation.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod WebViewProvider;
// pub use self::WebViewProvider::WebViewProvider;

// --- Data Transfer Objects ---
pub mod DTO;

// --- Effect Constructors ---
// Placeholders for future effects like CreateWebViewPanel, SetWebViewHTML, etc.
