//! # Error Module
//!
//! Defines the universal, structured error type for the application and
//! aggregates all error-related exports. This ensures consistent and robust
//! error handling across all services and operations.

#![allow(non_snake_case, non_camel_case_types)]

pub mod CommonError;

// --- Public Re-exports ---

// /// The primary, comprehensive error enum for all operations within the
// Common /// crate and the applications that use it.
// /// @see CommonError
// pub use self::CommonError::CommonError::CommonError;
