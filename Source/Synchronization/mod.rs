//! # Synchronization Service
//!
//! Defines the abstract contract for the User Data Synchronization service,
//! responsible for synchronizing settings, snippets, UI state, and other user
//! data across different instances of the application.

// --- Trait Definition ---
/// Trait for synchronizing user data across application instances.
pub mod SynchronizationProvider;
