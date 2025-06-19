//! # Command Service
//!
//! This module defines the abstract contract for the Command service. It
//! includes the `CommandExecutor` trait, which outlines the capabilities for
//! command management, and the `ActionEffect` constructors for all
//! command-related operations.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod CommandExecutor;
// pub use self::CommandExecutor::CommandExecutor;

// --- Effect Constructors ---
pub mod ExecuteCommand;
pub mod GetAllCommands;
pub mod RegisterCommand;
pub mod UnregisterCommand;

// pub use self::ExecuteCommand::ExecuteCommand;
// pub use self::GetAllCommands::GetAllCommands;
// pub use self::RegisterCommand::RegisterCommand;
// pub use self::UnregisterCommand::UnregisterCommand;
