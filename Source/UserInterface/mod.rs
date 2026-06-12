//! # UserInterface Service
//!
//! Defines the abstract contract for the UserInterface service, including the
//! `UserInterfaceProvider` trait, related Data Transfer Objects (DTOs), and
//! `ActionEffect` constructors for every UI-related operation.

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
