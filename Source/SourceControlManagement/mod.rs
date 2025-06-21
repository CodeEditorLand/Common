// File: Common/Source/SourceControlManagement/mod.rs
// Role: Public module interface for the Source Control Management (SCM) service
// contract. Responsibilities:
//   - Expose all necessary traits, DTOs, and effect constructors related to
//     SCM.

//! # SourceControlManagement Service
//!
//! This module defines the abstract contract for the Source Control Management
//! (SCM) service, which is responsible for integrating with version control
//! systems like Git.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod SourceControlManagementProvider;
// pub use self::SourceControlManagementProvider::SourceControlManagementProvider;

// --- Data Transfer Objects ---
pub mod DTO;

// --- Effect Constructors ---
// pub mod CreateSourceControl;
// pub mod DisposeSourceControl;
// pub mod RegisterInputBox;
// pub mod UpdateSourceControl;
// pub mod UpdateSourceControlGroup;

// pub use self::{
// 	CreateSourceControl::CreateSourceControl,
// 	DisposeSourceControl::DisposeSourceControl,
// 	RegisterInputBox::RegisterInputBox,
// 	UpdateSourceControl::UpdateSourceControl,
// 	UpdateSourceControlGroup::UpdateSourceControlGroup,
// };
