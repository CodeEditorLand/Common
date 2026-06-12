// File: Common/Source/Terminal/mod.rs
// Role: Public module interface for the Terminal service contract.
// Responsibilities:
//   - Expose all necessary traits and effect constructors related to the
//     integrated terminal.

//! # Terminal Service
//!
//! Defines the abstract contract for the integrated Terminal service, including
//! the `TerminalProvider` trait and `ActionEffect` constructors for every
//! terminal-related operation.

// --- Trait Definition ---
/// Trait for creating and managing terminal instances.
pub mod TerminalProvider;

// --- Effect Constructors ---
/// Effect constructor for creating a new terminal.
pub mod CreateTerminal;
