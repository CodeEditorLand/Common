

//
// @module dto (StatusBar)
// @description Aggregates and re-exports all Data Transfer Objects (DTOs)
// related to the Status Bar API.
//

#![allow(non_snake_case, non_camel_case_types)]

mod StatusBarAlignmentDto;
mod StatusBarEntryDto;

pub use self::StatusBarAlignmentDto::StatusBarAlignmentDto;
pub use self::StatusBarEntryDto::StatusBarEntryDto;
