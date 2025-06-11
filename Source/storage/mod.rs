

//
// @module storage
// @description This module defines the abstract contract for the Storage service,
// which provides Memento-style persistent key-value storage. It includes the
// `StorageProvider` trait and the `ActionEffect` constructors for storage operations.
//

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod StorageProvider;
pub use self::StorageProvider::StorageProvider;

// --- Effect Constructors ---
mod GetStorageItem;
mod SetStorageItem;

pub use self::GetStorageItem::GetStorageItem;
pub use self::SetStorageItem::SetStorageItem;
