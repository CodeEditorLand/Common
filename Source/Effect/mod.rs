//! # Effect Module
//!
//! Defines the core components of the declarative, effects-based architecture.
//! It provides the `ActionEffect` data structure, the `ApplicationRunTime`
//! trait for executing effects, and a convenience helper function.

#![allow(non_snake_case, non_camel_case_types)]

// --- Sub-modules ---
pub mod ActionEffect;

pub mod ApplicationRunTime;

pub mod ExecuteEffect;
