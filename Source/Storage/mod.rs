//! # Storage Service
//!
//! This module defines the abstract contract for the Storage service, which
//! provides Memento-style persistent key-value storage for extensions. It
//! includes the `StorageProvider` trait and the `ActionEffect` constructors
//! for all storage operations.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod StorageProvider;
// pub use self::StorageProvider::StorageProvider;

// --- Effect Constructors ---
pub mod GetStorageItem;
pub mod SetStorageItem;

// pub use self::{GetStorageItem::GetStorageItem, SetStorageItem::SetStorageItem};
