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

// --- Trait Definition ---
pub mod LanguageFeatureProviderRegistry;

// --- Data Transfer Objects ---
pub mod DTO;

// --- Effect Constructors ---

// Provider Management
pub mod RegisterProvider;

pub mod UnregisterProvider;

// Feature Invocation
pub mod ProvideCompletions;
pub mod ProvideHover;
pub mod ProvideDefinition;
pub mod ProvideReferences;
pub mod ProvideDocumentSymbols;
pub mod ProvideWorkspaceSymbols;
pub mod ProvideRenameEdits;
pub mod ProvideDocumentFormatting;
pub mod ProvideSignatureHelp;
pub mod ProvideCodeLenses;
pub mod ProvideFoldingRanges;
pub mod ProvideSelectionRanges;
pub mod ProvideSemanticTokens;
pub mod ProvideInlayHints;
pub mod ProvideTypeHierarchy;
pub mod ProvideCallHierarchy;
pub mod ProvideLinkedEditingRanges;
pub mod ProvideOnTypeFormatting;
pub mod ProvideDocumentHighlights;
pub mod ProvideCodeActions;
