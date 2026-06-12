// File: Common/Source/LanguageFeature/mod.rs
// Role: Public module interface for the Language Feature service contract.
// Responsibilities:
//   - Expose all necessary traits, DTOs, and effect constructors related to
//     language features.

//! # LanguageFeature Service
//!
//! Defines the abstract contract for all language intelligence services,
//! including the `LanguageFeatureProviderRegistry` trait, related Data Transfer
//! Objects (DTOs), and `ActionEffect` constructors for every language feature
//! operation. This is the largest and most complex service contract in the
//! application.

// --- Trait Definition ---
/// Trait for registering and unregistering language feature providers.
pub mod LanguageFeatureProviderRegistry;

// --- Data Transfer Objects ---
/// DTOs for language feature APIs.
pub mod DTO;

// --- Effect Constructors ---

// Provider Management
/// Effect constructor for registering a language feature provider.
pub mod RegisterProvider;

/// Effect constructor for unregistering a language feature provider.
pub mod UnregisterProvider;

// Feature Invocation
/// Effect constructor for providing code completions.
pub mod ProvideCompletions;

/// Effect constructor for providing hover information.
pub mod ProvideHover;

/// Effect constructor for providing go-to-definition results.
pub mod ProvideDefinition;

/// Effect constructor for providing reference locations.
pub mod ProvideReferences;

/// Effect constructor for providing document symbols.
pub mod ProvideDocumentSymbols;

/// Effect constructor for providing workspace symbols.
pub mod ProvideWorkspaceSymbols;

/// Effect constructor for providing rename edits.
pub mod ProvideRenameEdits;

/// Effect constructor for providing document formatting.
pub mod ProvideDocumentFormatting;

/// Effect constructor for providing signature help.
pub mod ProvideSignatureHelp;

/// Effect constructor for providing code lens results.
pub mod ProvideCodeLenses;

/// Effect constructor for providing folding ranges.
pub mod ProvideFoldingRanges;

/// Effect constructor for providing selection ranges.
pub mod ProvideSelectionRanges;

/// Effect constructor for providing semantic tokens.
pub mod ProvideSemanticTokens;

/// Effect constructor for providing inlay hints.
pub mod ProvideInlayHints;

/// Effect constructor for providing type hierarchy.
pub mod ProvideTypeHierarchy;

/// Effect constructor for providing call hierarchy.
pub mod ProvideCallHierarchy;

/// Effect constructor for providing linked editing ranges.
pub mod ProvideLinkedEditingRanges;

/// Effect constructor for providing on-type formatting.
pub mod ProvideOnTypeFormatting;

/// Effect constructor for providing document highlights.
pub mod ProvideDocumentHighlights;

/// Effect constructor for providing code actions.
pub mod ProvideCodeActions;
