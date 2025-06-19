//! # LanguageFeature DTO Module
//!
//! Aggregates and re-exports all Data Transfer Objects (DTOs) related to the
//! various Language Feature APIs (e.g., completions, hovers, symbols).

#![allow(non_snake_case, non_camel_case_types)]

pub mod HoverResultDTO;
pub mod IMarkdownStringDTO;
pub mod PositionDTO;
pub mod RangeDTO;
// ... many more DTOs will be added here

// pub use self::{
// 	HoverResultDTO::HoverResultDTO,
// 	IMarkdownStringDTO::IMarkdownStringDTO,
// 	PositionDTO::PositionDTO,
// 	RangeDTO::RangeDTO,
// };
