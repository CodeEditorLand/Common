

//
// @module webview
// @description This module defines the abstract contract for the Webview service.
// It includes the `WebviewProvider` trait, all related DTOs, and the `ActionEffect`
// constructors for every webview operation.
//

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod WebviewProvider;
pub use self::WebviewProvider::WebviewProvider;

// --- Data Transfer Objects ---
pub mod dto;

// --- Effect Constructors ---
mod CreateWebviewPanel;
mod DisposeWebview;
mod PostMessageToWebview;
mod RevealWebviewPanel;
mod SetWebviewHtml;
mod SetWebviewIconPath;
mod SetWebviewTitle;

pub use self::CreateWebviewPanel::CreateWebviewPanel;
pub use self::DisposeWebview::DisposeWebview;
pub use self::PostMessageToWebview::PostMessageToWebview;
pub use self::RevealWebviewPanel::RevealWebviewPanel;
pub use self::SetWebviewHtml::SetWebviewHtml;
pub use self::SetWebviewIconPath::SetWebviewIconPath;
pub use self::SetWebviewTitle::SetWebviewTitle;
