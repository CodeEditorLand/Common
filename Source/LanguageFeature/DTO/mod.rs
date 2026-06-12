//! # LanguageFeature DTO Module
//!
//! Aggregates and re-exports all Data Transfer Objects (DTOs) related to the
//! various Language Feature APIs (e.g., completions, hovers, symbols).

/// DTO for completion request context (trigger kind, character).
pub mod CompletionContextDTO;

/// DTO for a single completion item.
pub mod CompletionItemDTO;

/// DTO for a list of completion results.
pub mod CompletionListDTO;

/// DTO for hover result contents.
pub mod HoverResultDTO;

/// DTO for markdown-formatted strings in tooltips.
pub mod IMarkdownStringDTO;

/// DTO for a source-code location (URI + range).
pub mod LocationDTO;

/// DTO for a zero-based position in a document.
pub mod PositionDTO;

/// Enum for language feature provider types.
pub mod ProviderType;

/// DTO for a range between two positions in a document.
pub mod RangeDTO;

/// DTO for a single text edit (range + replacement text).
pub mod TextEditDTO;
