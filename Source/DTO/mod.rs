//! # Global DTO Module
//!
//! A top-level module that re-exports all Data Transfer Objects (DTOs) from the
//! various service modules for convenient access across the application.
//! It also contains DTOs that are shared across multiple services.

#![allow(non_snake_case, non_camel_case_types)]

pub mod WorkSpaceEditDTO;

// // Re-export shared DTOs
// pub use self::WorkSpaceEditDTO::WorkSpaceEditDTO;

// // Re-export service-specific DTOs
// pub use crate::Configuration::DTO::*;
// pub use crate::FileSystem::DTO::*;
// pub use crate::IPC::DTO::*;
// pub use crate::LanguageFeature::DTO::*;
// pub use crate::SourceControlManagement::DTO::*;
// pub use crate::StatusBar::DTO::*;
// pub use crate::TreeView::DTO::*;
// pub use crate::UserInterface::DTO::*;
// pub use crate::WebView::DTO::*;
