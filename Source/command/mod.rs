

/**
 * @module command
 * @description This module defines the abstract contract for the Command service.
 * It includes the `CommandExecutor` trait and the `ActionEffect` constructors for
 * all command operations.
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod CommandExecutor;
pub use self::CommandExecutor::CommandExecutor;

// --- Effect Constructors ---
mod ExecuteCommand;
mod GetAllCommands;
mod RegisterCommand;
mod UnregisterCommand;

pub use self::ExecuteCommand::ExecuteCommand;
pub use self::GetAllCommands::GetAllCommands;
pub use self::RegisterCommand::RegisterCommand;
pub use self::UnregisterCommand::UnregisterCommand;
