//! # LanguageFeature DTO Module
//!
//! Aggregates and re-exports all Data Transfer Objects (DTOs) related to the
//! various Language Feature APIs (e.g., completions, hovers, symbols).

#![allow(non_snake_case, non_camel_case_types)]

pub mod CompletionContextDTO;
pub mod CompletionItemDTO;
pub mod CompletionListDTO;
pub mod HoverResultDTO;
pub mod IMarkdownStringDTO;
pub mod LocationDTO;
pub mod PositionDTO;
pub mod ProviderType;
pub mod RangeDTO;
pub mod TextEditDTO;
