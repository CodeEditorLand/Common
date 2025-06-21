// File: Common/Source/LanguageFeature/mod.rs
// Role: Public module interface for the Language Feature service contract.
// Responsibilities:
//   - Expose all necessary traits, DTOs, and effect constructors related to
//     language features.

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

// Feature Invocation
// pub mod PrepareRename;
// pub mod ProvideCodeActions;
// pub mod ProvideCodeLenses;
pub mod ProvideCompletions;
// pub mod ProvideDefinition;
// pub mod ProvideDocumentFormattingEdits;
// pub mod ProvideDocumentHighlights;
// pub mod ProvideDocumentLinks;
// pub mod ProvideDocumentRangeFormattingEdits;
pub mod ProvideHover;
// pub mod ProvideReferences;

// pub use self::{
// 	PrepareRename::PrepareRename,
// 	ProvideCodeActions::ProvideCodeActions,
// 	ProvideCodeLenses::ProvideCodeLenses,
// 	ProvideCompletions::ProvideCompletions,
// 	ProvideDefinition::ProvideDefinition,
// 	ProvideDocumentFormattingEdits::ProvideDocumentFormattingEdits,
// 	ProvideDocumentHighlights::ProvideDocumentHighlights,
// 	ProvideDocumentLinks::ProvideDocumentLinks,
// 	ProvideDocumentRangeFormattingEdits::ProvideDocumentRangeFormattingEdits,
// 	ProvideHover::ProvideHover,
// 	ProvideReferences::ProvideReferences,
// 	RegisterProvider::RegisterProvider,
// 	UnregisterProvider::UnregisterProvider,
// };
