//! # UserInterface Service
//!
//! This module defines the abstract contract for the UserInterface service. It
//! includes the `UserInterfaceProvider` trait, all related Data Transfer
//! Objects (DTOs), and the `ActionEffect` constructors for every
//! UI-related operation.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod UserInterfaceProvider;

// --- Data Transfer Objects ---
pub mod DTO;

// --- Effect Constructors ---
pub mod ShowInputBox;
pub mod ShowMessage;
pub mod ShowOpenDialog;
pub mod ShowQuickPick;
pub mod ShowSaveDialog;
