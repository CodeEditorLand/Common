//! # Command Service
//!
//! Defines the abstract contract for the Command service, including
//! the `CommandExecutor` trait and `ActionEffect` constructors for all
//! command-related operations.

// --- Trait Definition ---
/// Trait for registering, executing, and managing commands.
pub mod CommandExecutor;

// --- Effect Constructors ---
/// Effect constructor for executing a command by ID.
pub mod ExecuteCommand;

/// Effect constructor for retrieving all registered commands.
pub mod GetAllCommands;

/// Effect constructor for registering a new command.
pub mod RegisterCommand;

/// Effect constructor for unregistering an existing command.
pub mod UnregisterCommand;
