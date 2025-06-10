

/**
 * @module language_feature
 * @description This module defines the abstract contract for all language intelligence
 * services. It includes the main `LanguageFeatureProviderRegistry` trait, all
 * related DTOs, and the `ActionEffect` constructors for every language feature
 * operation. This is the largest and most complex service contract in the application.
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod LanguageFeatureProviderRegistry;
pub use self::LanguageFeatureProviderRegistry::LanguageFeatureProviderRegistry;

// --- Data Transfer Objects ---
pub mod dto;

// --- Effect Constructors ---

// Provider Management
mod RegisterProvider;
mod UnregisterProvider;

pub use self::RegisterProvider::RegisterProvider;
pub use self::UnregisterProvider::UnregisterProvider;

// Feature Invocation
mod PrepareCallHierarchy;
mod PrepareRename;
mod PrepareTypeHierarchy;
mod ProvideCallHierarchyIncomingCalls;
mod ProvideCallHierarchyOutgoingCalls;
mod ProvideCodeActions;
mod ProvideCodeLenses;
mod ProvideCompletions;
mod ProvideDocumentFormattingEdits;
mod ProvideDocumentHighlights;
mod ProvideDocumentLinks;
mod ProvideDocumentRangeFormattingEdits;
mod ProvideDocumentRangeSemanticTokens;
mod ProvideDocumentSemanticTokens;
mod ProvideDocumentSemanticTokensEdits;
mod ProvideDocumentSymbols;
mod ProvideFoldingRanges;
mod ProvideHover;
mod ProvideInlayHints;
mod ProvideLinkedEditingRanges;
mod ProvideOnTypeFormattingEdits;
mod ProvideReferences;
mod ProvideRenameEdits;
mod ProvideSelectionRanges;
mod ProvideSignatureHelp;
mod ProvideTypeHierarchySubtypes;
mod ProvideTypeHierarchySupertypes;
mod ProvideWorkspaceSymbols;
mod ResolveCodeAction;
mod ResolveCodeLens;
mod ResolveCompletionItem;
mod ResolveDocumentLink;
mod ResolveInlayHint;

pub use self::PrepareCallHierarchy::PrepareCallHierarchy;
pub use self::PrepareRename::PrepareRename;
pub use self::PrepareTypeHierarchy::PrepareTypeHierarchy;
pub use self::ProvideCallHierarchyIncomingCalls::ProvideCallHierarchyIncomingCalls;
pub use self::ProvideCallHierarchyOutgoingCalls::ProvideCallHierarchyOutgoingCalls;
pub use self::ProvideCodeActions::ProvideCodeActions;
pub use self::ProvideCodeLenses::ProvideCodeLenses;
pub use self::ProvideCompletions::ProvideCompletions;
pub use self::ProvideDocumentFormattingEdits::ProvideDocumentFormattingEdits;
pub use self::ProvideDocumentHighlights::ProvideDocumentHighlights;
pub use self::ProvideDocumentLinks::ProvideDocumentLinks;
pub use self::ProvideDocumentRangeFormattingEdits::ProvideDocumentRangeFormattingEdits;
pub use self::ProvideDocumentRangeSemanticTokens::ProvideDocumentRangeSemanticTokens;
pub use self::ProvideDocumentSemanticTokens::ProvideDocumentSemanticTokens;
pub use self::ProvideDocumentSemanticTokensEdits::ProvideDocumentSemanticTokensEdits;
pub use self::ProvideDocumentSymbols::ProvideDocumentSymbols;
pub use self::ProvideFoldingRanges::ProvideFoldingRanges;
pub use self::ProvideHover::ProvideHover;
pub use self::ProvideInlayHints::ProvideInlayHints;
pub use self::ProvideLinkedEditingRanges::ProvideLinkedEditingRanges;
pub use self::ProvideOnTypeFormattingEdits::ProvideOnTypeFormattingEdits;
pub use self::ProvideReferences::ProvideReferences;
pub use self::ProvideRenameEdits::ProvideRenameEdits;
pub use self::ProvideSelectionRanges::ProvideSelectionRanges;
pub use self::ProvideSignatureHelp::ProvideSignatureHelp;
pub use self::ProvideTypeHierarchySubtypes::ProvideTypeHierarchySubtypes;
pub use self::ProvideTypeHierarchySupertypes::ProvideTypeHierarchySupertypes;
pub use self::ProvideWorkspaceSymbols::ProvideWorkspaceSymbols;
pub use self::ResolveCodeAction::ResolveCodeAction;
pub use self::ResolveCodeLens::ResolveCodeLens;
pub use self::ResolveCompletionItem::ResolveCompletionItem;
pub use self::ResolveDocumentLink::ResolveDocumentLink;
pub use self::ResolveInlayHint::ResolveInlayHint;
