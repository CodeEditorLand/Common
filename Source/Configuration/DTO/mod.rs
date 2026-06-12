//! # Configuration DTO Module
//!
//! Aggregates and re-exports all Data Transfer Objects (DTOs) related to the
//! Configuration service.

// --- DTO Definitions ---
/// DTO for initializing configuration values.
pub mod ConfigurationInitializationDTO;

/// DTO for specifying configuration overrides (resource, language scopes).
pub mod ConfigurationOverridesDTO;

/// Enum for configuration scope (Default, User, Workspace, etc.).
pub mod ConfigurationScope;

/// Enum for identifying the target of a configuration update.
pub mod ConfigurationTarget;

/// DTO for the result of a configuration inspection.
pub mod InspectResultDataDTO;
