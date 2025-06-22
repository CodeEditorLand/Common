// File: Common/Source/Storage/mod.rs
// Role: Public module interface for the Storage service contract.
// Responsibilities:
//   - Expose all necessary traits and effect constructors related to storage.
//   - This contract includes both a high-performance batch-oriented API
//     (`GetAllStorage`, `SetAllStorage`) and a legacy per-key API.

//! # Storage Service
//!
//! This module defines the abstract contract for the Storage service, which
//! provides Memento-style persistent key-value storage for extensions. It
//! includes the `StorageProvider` trait and the `ActionEffect` constructors
//! for all storage operations.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod StorageProvider;

// --- Effect Constructors ---
// Legacy per-key effects
pub mod GetStorageItem;
pub mod SetStorageItem;
