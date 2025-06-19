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

// --- Public Re-exports ---

// /// The core struct representing an asynchronous operation as a first-class
// /// value.
// /// @see ActionEffect
// pub use self::ActionEffect::ActionEffect;
// /// The core trait for any "engine" capable of executing `ActionEffect`s.
// /// @see ApplicationRunTime
// pub use self::ApplicationRunTime::ApplicationRunTime;
// /// A generic helper function for a more ergonomic way to run
// `ActionEffect`s. /// @see ExecuteEffect
// pub use self::ExecuteEffect::ExecuteEffect;
