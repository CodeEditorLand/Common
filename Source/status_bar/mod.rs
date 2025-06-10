

/**
 * @module status_bar
 * @description This module defines the abstract contract for the Status Bar service.
 * It includes the `StatusBarProvider` trait, all related DTOs, and the `ActionEffect`
 * constructors for every status bar operation.
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod StatusBarProvider;
pub use self::StatusBarProvider::StatusBarProvider;

// --- Data Transfer Objects ---
pub mod dto;

// --- Effect Constructors ---
mod SetEntry;
mod DisposeEntry;
mod ProvideTooltip;

pub use self::SetEntry::SetEntry;
pub use self::DisposeEntry::DisposeEntry;
pub use self::ProvideTooltip::ProvideTooltip;
