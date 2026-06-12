//! # UserInterface Service
//!
//! This module defines the abstract contract for the UserInterface service. It
//! includes the `UserInterfaceProvider` trait, all related Data Transfer
//! Objects (DTOs), and the `ActionEffect` constructors for every
//! UI-related operation.

// --- Trait Definition ---
/// Trait for showing dialogs, messages, quick picks, and input boxes.
pub mod UserInterfaceProvider;

// --- Data Transfer Objects ---
/// DTOs for the UserInterface service.
pub mod DTO;

// --- Effect Constructors ---
/// Effect constructor for showing an input box.
pub mod ShowInputBox;

/// Effect constructor for showing a message notification.
pub mod ShowMessage;

/// Effect constructor for showing an open-file dialog.
pub mod ShowOpenDialog;

/// Effect constructor for showing a quick pick list.
pub mod ShowQuickPick;

/// Effect constructor for showing a save-file dialog.
pub mod ShowSaveDialog;
