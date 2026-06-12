//! # Environment Module
//!
//! Defines the core architectural traits for dependency injection and
//! environmental context within the application. This module provides the
//! foundational components (`Environment`, `Requires`, `HasEnvironment`) that
//! enable the entire effects-based system to function in a decoupled and
//! testable manner.

// --- Core Trait Definitions ---
/// The base `Environment` trait - a marker for capability-providing types.
pub mod Environment;

/// The `HasEnvironment` trait for accessing the environment from effects.
pub mod HasEnvironment;

/// The `Requires` trait declaring what capability an effect needs.
pub mod Requires;
