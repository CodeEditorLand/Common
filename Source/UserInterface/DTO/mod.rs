//! # UserInterface DTO Module
//!
//! Aggregates and re-exports all Data Transfer Objects (DTOs) related to the
//! user interface service, such as dialogs, messages, and quick picks.

/// DTO for dialog window options (title, buttons, detail).
pub mod DialogOptionsDTO;

/// DTO for file dialog filter definitions.
pub mod FileFilterDTO;

/// DTO for input box options (prompt, placeholder, validation).
pub mod InputBoxOptionsDTO;

/// DTO for message dialog options (type, buttons, detail).
pub mod MessageOptionsDTO;

/// Enum for message severity (Info, Warning, Error).
pub mod MessageSeverity;

/// DTO for open-file dialog options (canSelectFiles, canSelectFolders).
pub mod OpenDialogOptionsDTO;

/// DTO for a single quick pick item.
pub mod QuickPickItemDTO;

/// DTO for quick pick options (placeholder, matchOnDescription).
pub mod QuickPickOptionsDTO;

/// DTO for save-file dialog options (defaultUri, filters).
pub mod SaveDialogOptionsDTO;
