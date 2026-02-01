//! # Secret Service
//!
//! This module defines the abstract contract for the Secret service, which
//! provides secure storage capabilities for extensions, typically by
//! integrating with the operating system's keychain or credential store. It
//! includes the `SecretProvider` trait and the `ActionEffect` constructors for
//! all secret management operations.

// --- Trait Definition ---
pub mod SecretProvider;

// --- Effect Constructors ---
pub mod DeleteSecret;

pub mod GetSecret;

pub mod StoreSecret;
