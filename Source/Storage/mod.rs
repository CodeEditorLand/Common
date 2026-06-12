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

// --- Trait Definition ---
/// Trait for persistent key-value storage for extensions (Memento API).
pub mod StorageProvider;

// --- Effect Constructors ---
// Legacy per-key effects
/// Effect constructor for retrieving a single storage item by key.
pub mod GetStorageItem;

/// Effect constructor for storing a single item by key.
pub mod SetStorageItem;
