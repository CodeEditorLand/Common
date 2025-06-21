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

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod WebViewProvider;
// pub use self::WebViewProvider::WebViewProvider;

// --- Data Transfer Objects ---
pub mod DTO;

// --- Effect Constructors ---
// pub mod CreateWebViewPanel;
// pub mod DisposeWebViewPanel;
// pub mod PostMessageToWebView;
// pub mod RevealWebViewPanel;
// pub mod SetWebViewHTML;
// pub mod SetWebViewOptions;

// pub use self::{
// 	CreateWebViewPanel::CreateWebViewPanel,
// 	DisposeWebViewPanel::DisposeWebViewPanel,
// 	PostMessageToWebView::PostMessageToWebView,
// 	RevealWebViewPanel::RevealWebViewPanel,
// 	SetWebViewHTML::SetWebViewHTML,
// 	SetWebViewOptions::SetWebViewOptions,
// };
