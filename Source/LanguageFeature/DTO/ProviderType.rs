//! # ProviderType DTO
//!
//! Defines the enum that identifies each type of language feature provider.

use std::fmt;

use serde::{Deserialize, Serialize};

/// An enum that provides a unique identifier for each type of language feature.
/// This is used to register and query for specific provider implementations.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ProviderType {
	Completion = 0,

	Hover = 1,

	SignatureHelp = 2,

	Definition = 3,

	TypeDefinition = 4,

	Implementation = 5,

	References = 6,

	DocumentHighlight = 7,

	DocumentSymbol = 8,

	WorkspaceSymbol = 9,

	CodeAction = 10,

	CodeLens = 11,

	DocumentFormatting = 12,

	DocumentRangeFormatting = 13,

	OnTypeFormatting = 14,

	Rename = 15,

	DocumentLink = 16,

	Color = 17,

	FoldingRange = 18,

	Declaration = 19,

	SelectionRange = 20,

	InlayHint = 21,

	CallHierarchy = 22,

	SemanticTokens = 23,

	LinkedEditingRange = 24,

	TypeHierarchy = 25,

	EvaluatableExpression = 26,

	InlineValues = 27,
}

impl fmt::Display for ProviderType {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result { write!(f, "{:?}", self) }
}
