//! # LanguageFeature Service
//!
//! This module defines the abstract contract for all language intelligence
//! services. It includes the main `LanguageFeatureProviderRegistry` trait, all
//! related Data Transfer Objects (DTOs), and the `ActionEffect` constructors
//! for every language feature operation. This is the largest and most complex
//! service contract in the application.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod LanguageFeatureProviderRegistry;
// pub use self::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;

// --- Data Transfer Objects ---
pub mod DTO;

// --- Effect Constructors ---

// Provider Management
pub mod RegisterProvider;
pub mod UnregisterProvider;

// pub use self::{RegisterProvider::RegisterProvider, UnregisterProvider::UnregisterProvider};

// Feature Invocation (Placeholders for the many provider effects)
pub mod ProvideCompletions;
pub mod ProvideHover;
// ... more to come in subsequent batches

// pub use self::{ProvideCompletions::ProvideCompletions, ProvideHover::ProvideHover};
