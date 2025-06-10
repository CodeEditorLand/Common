

/**
 * @module environment (Common)
 * @description This module defines the core architectural traits for dependency
 * injection and environment context within the application. It provides the
 * foundation for the entire effect system.
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Core Trait Definitions ---
mod Environment;
mod HasEnvironment;
mod Requires;

pub use self::Environment::Environment;
pub use self::HasEnvironment::HasEnvironment;
pub use self::Requires::Requires;
