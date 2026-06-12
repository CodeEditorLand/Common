// File: Common/Source/Configuration/mod.rs
// Role: Public module interface for the Configuration service contract.
// Responsibilities:
//   - Expose all necessary traits, DTOs, and effect constructors related to
//     configuration.

//! # Configuration Service
//!
//! Defines the abstract contract for the Configuration service, including the
//! `ConfigurationProvider` and `ConfigurationInspector` traits, related Data
//! Transfer Objects (DTOs), and `ActionEffect` constructors for all
//! configuration-related operations.

// --- Trait Definitions ---
/// Trait for inspecting configuration metadata (defaults, descriptions).
pub mod ConfigurationInspector;

/// Trait for reading and writing configuration values.
pub mod ConfigurationProvider;

// --- Data Transfer Objects ---
/// DTOs for the Configuration service.
pub mod DTO;

// --- Effect Constructors ---
/// Effect constructor for retrieving configuration values.
pub mod GetConfiguration;

/// Effect constructor for inspecting configuration metadata.
pub mod InspectConfiguration;

/// Effect constructor for updating configuration values.
pub mod UpdateConfiguration;
