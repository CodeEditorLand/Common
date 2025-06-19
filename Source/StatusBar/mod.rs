//! # StatusBar Service
//!
//! This module defines the abstract contract for the StatusBar service. It
//! includes the `StatusBarProvider` trait, all related Data Transfer Objects
//! (DTOs), and will include `ActionEffect` constructors for status bar
//! operations.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod StatusBarProvider;
// pub use self::StatusBarProvider::StatusBarProvider;

// --- Data Transfer Objects ---
pub mod DTO;

// --- Effect Constructors ---
// Placeholders for future effects like SetEntry, DisposeEntry, etc.
