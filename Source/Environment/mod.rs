//! # Environment Module
//!
//! Defines the core architectural traits for dependency injection and
//! environmental context within the application. This module provides the
//! foundational components (`Environment`, `Requires`, `HasEnvironment`) that
//! enable the entire effects-based system to function in a decoupled and
//! testable manner.

#![allow(non_snake_case, non_camel_case_types)]

// --- Core Trait Definitions ---
pub mod Environment;
pub mod HasEnvironment;
pub mod Requires;

// --- Public Re-exports ---

// /// A marker trait for any struct that represents an application's
// environment. /// @see Environment
// pub use self::Environment::Environment::Environment;
// /// A trait for any type that holds and provides access to an `Environment`.
// /// @see HasEnvironment
// pub use self::HasEnvironment::HasEnvironment;
// /// The core dependency injection trait that allows an `Environment` to
// provide /// a specific capability.
// /// @see Requires
// pub use self::Requires::Requires;
