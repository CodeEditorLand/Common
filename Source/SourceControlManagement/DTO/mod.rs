//! # SourceControlManagement DTO Module
//!
//! Aggregates and re-exports all Data Transfer Objects (DTOs) related to the
//! Source Control Management (SCM) API.

#![allow(non_snake_case, non_camel_case_types)]

pub mod SourceControlCreateDTO;
pub mod SourceControlGroupUpdateDTO;
pub mod SourceControlInputBoxDTO;
pub mod SourceControlManagementGroupDTO;
pub mod SourceControlManagementProviderDTO;
pub mod SourceControlManagementResourceDTO;
pub mod SourceControlUpdateDTO;

// pub use self::{
// 	SourceControlManagementGroupDTO::SourceControlManagementGroupDTO,
// 	SourceControlManagementProviderDTO::SourceControlManagementProviderDTO,
// 	SourceControlManagementResourceDTO::SourceControlManagementResourceDTO,
// };
