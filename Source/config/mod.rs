

/**
 * @module config
 * @description This module defines the abstract contract for the Configuration service.
 * It includes the `ConfigProvider` and `ConfigInspector` traits, all related DTOs,
 * and the `ActionEffect` constructors for all configuration operations.
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definitions ---
mod ConfigInspector;
mod ConfigProvider;

pub use self::ConfigInspector::ConfigInspector;
pub use self::ConfigProvider::ConfigProvider;

// --- Data Transfer Objects ---
pub mod dto;

// --- Effect Constructors ---
mod GetConfiguration;
mod InspectConfiguration;
mod UpdateConfiguration;

pub use self::GetConfiguration::GetConfiguration;
pub use self::InspectConfiguration::InspectConfiguration;
pub use self::UpdateConfiguration::UpdateConfiguration;
