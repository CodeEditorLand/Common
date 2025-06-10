

/**
 * @module ui
 * @description This module defines the abstract contract for the UI service.
 * It includes the `UiProvider` trait, all related DTOs, and the `ActionEffect`
 * constructors for every UI-related operation.
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod UiProvider;
pub use self::UiProvider::UiProvider;

// --- Data Transfer Objects ---
pub mod dto;

// --- Effect Constructors ---
mod ShowInputBox;
mod ShowMessage;
mod ShowOpenDialog;
mod ShowQuickPick;
mod ShowSaveDialog;

pub use self::ShowInputBox::ShowInputBox;
pub use self::ShowMessage::ShowMessage;
pub use self::ShowOpenDialog::ShowOpenDialog;
pub use self::ShowQuickPick::ShowQuickPick;
pub use self::ShowSaveDialog::ShowSaveDialog;
