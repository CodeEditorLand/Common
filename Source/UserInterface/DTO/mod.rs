//! # UserInterface DTO Module
//!
//! Aggregates and re-exports all Data Transfer Objects (DTOs) related to the
//! user interface service, such as dialogs, messages, and quick picks.

#![allow(non_snake_case, non_camel_case_types)]

pub mod DialogOptionsDTO;

pub mod FileFilterDTO;

pub mod InputBoxOptionsDTO;

pub mod MessageOptionsDTO;

pub mod MessageSeverity;

pub mod OpenDialogOptionsDTO;

pub mod QuickPickItemDTO;

pub mod QuickPickOptionsDTO;

pub mod SaveDialogOptionsDTO;
