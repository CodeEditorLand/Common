//! # Effect Module
//!
//! Defines the core components of the declarative, effects-based architecture.
//! It provides the `ActionEffect` data structure, the `ApplicationRunTime`
//! trait for executing effects, and a convenience helper function.

// --- Sub-modules ---
/// The core `ActionEffect` struct - a first-class async operation value.
pub mod ActionEffect;

/// The `ApplicationRunTime` trait for providing capabilities and executing
/// effects.
pub mod ApplicationRunTime;

/// Convenience function for executing an `ActionEffect` via the runtime.
pub mod ExecuteEffect;
