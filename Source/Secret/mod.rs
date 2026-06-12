//! # Secret Service
//!
//! Defines the abstract contract for the Secret service, which provides secure
//! storage capabilities for extensions, typically by integrating with the
//! operating system's keychain or credential store. Includes the
//! `SecretProvider` trait and `ActionEffect` constructors for all secret
//! management operations.

// --- Trait Definition ---
/// Trait for accessing the OS keychain or credential store.
pub mod SecretProvider;

// --- Effect Constructors ---
/// Effect constructor for deleting a stored secret.
pub mod DeleteSecret;

/// Effect constructor for retrieving a stored secret.
pub mod GetSecret;

/// Effect constructor for storing a new secret.
pub mod StoreSecret;
