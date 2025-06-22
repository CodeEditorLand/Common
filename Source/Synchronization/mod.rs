//! # Synchronization Service
//!
//! This module defines the abstract contract for the User Data Synchronization
//! service. This service is responsible for synchronizing settings, snippets,
//! UI state, and other user data across different instances of the application.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod SynchronizationProvider;
