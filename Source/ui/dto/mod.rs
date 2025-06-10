

/**
 * @module dto (UI)
 * @description Aggregates and re-exports all Data Transfer Objects (DTOs)
 * related to the user interface service, such as dialogs, messages, and quick picks.
 */

#![allow(non_snake_case, non_camel_case_types)]

mod DialogOptionsDto;
mod FileFilterDto;
mod InputBoxOptionsDto;
mod MessageOptionsDto;
mod MessageSeverity;
mod OpenDialogOptionsDto;
mod QuickPickItemDto;
mod QuickPickOptionsDto;
mod SaveDialogOptionsDto;

pub use self::DialogOptionsDto::DialogOptionsDto;
pub use self::FileFilterDto::FileFilterDto;
pub use self::InputBoxOptionsDto::InputBoxOptionsDto;
pub use self::MessageOptionsDto::MessageOptionsDto;
pub use self::MessageSeverity::MessageSeverity;
pub use self::OpenDialogOptionsDto::OpenDialogOptionsDto;
pub use self::QuickPickItemDto::QuickPickItemDto;
pub use self::QuickPickOptionsDto::QuickPickOptionsDto;
pub use self::SaveDialogOptionsDto::SaveDialogOptionsDto;
