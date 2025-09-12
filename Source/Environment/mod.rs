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
