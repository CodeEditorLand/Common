//! # Command Service
//!
//! This module defines the abstract contract for the Command service. It
//! includes the `CommandExecutor` trait, which outlines the capabilities for
//! command management, and the `ActionEffect` constructors for all
//! command-related operations.

// --- Trait Definition ---
pub mod CommandExecutor;

// --- Effect Constructors ---
pub mod ExecuteCommand;

pub mod GetAllCommands;

pub mod RegisterCommand;

pub mod UnregisterCommand;
