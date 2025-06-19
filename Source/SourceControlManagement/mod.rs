//! # SourceControlManagement Service
//!
//! This module defines the abstract contract for the Source Control Management
//! (SCM) service, which is responsible for integrating with version control
//! systems like Git.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod SourceControlManagementProvider;
// pub use self::SourceControlManagementProvider::SourceControlManagementProvider;

// --- Data Transfer Objects ---
pub mod DTO;

// --- Effect Constructors ---
// mod RegisterSourceControlManagementProvider; // Placeholder for future
// effects mod UpdateSourceControlManagementGroup;
