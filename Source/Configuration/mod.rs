// File: Common/Source/Configuration/mod.rs
// Role: Public module interface for the Configuration service contract.
// Responsibilities:
//   - Expose all necessary traits, DTOs, and effect constructors related to
//     configuration.

//! # Configuration Service
//!
//! This module defines the abstract contract for the Configuration service.
//! It includes the `ConfigurationProvider` and `ConfigurationInspector` traits,
//! all related Data Transfer Objects (DTOs), and the `ActionEffect`
//! constructors for all configuration-related operations.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definitions ---
pub mod ConfigurationInspector;
pub mod ConfigurationProvider;

// pub use self::{ConfigurationInspector::ConfigurationInspector,
// ConfigurationProvider::ConfigurationProvider};

// --- Data Transfer Objects ---
pub mod DTO;

// --- Effect Constructors ---
pub mod GetConfiguration;
pub mod InspectConfiguration;
pub mod UpdateConfiguration;

// pub use self::{
// 	GetConfiguration::GetConfiguration,
// 	InspectConfiguration::InspectConfiguration,
// 	UpdateConfiguration::UpdateConfiguration,
// };
