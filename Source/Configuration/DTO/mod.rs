//! # Configuration DTO Module
//!
//! Aggregates and re-exports all Data Transfer Objects (DTOs) related to the
//! Configuration service.

#![allow(non_snake_case, non_camel_case_types)]

// --- DTO Definitions ---
pub mod ConfigurationInitializationDTO;
pub mod ConfigurationOverridesDTO;
pub mod ConfigurationScope;
pub mod ConfigurationTarget;
pub mod InspectResultDataDTO;

// --- Public Re-exports ---
// pub use self::{
// 	ConfigurationInitializationDTO::ConfigurationInitializationDTO,
// 	ConfigurationOverridesDTO::ConfigurationOverridesDTO,
// 	ConfigurationScope::ConfigurationScope,
// 	ConfigurationTarget::ConfigurationTarget,
// 	InspectResultDataDTO::InspectResultDataDTO,
// };
