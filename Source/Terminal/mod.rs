// File: Common/Source/Terminal/mod.rs
// Role: Public module interface for the Terminal service contract.
// Responsibilities:
//   - Expose all necessary traits and effect constructors related to the
//     integrated terminal.

//! # Terminal Service
//!
//! This module defines the abstract contract for the integrated Terminal
//! service. It includes the `TerminalProvider` trait and the `ActionEffect`
//! constructors for every terminal-related operation.

// --- Trait Definition ---
pub mod TerminalProvider;

// --- Effect Constructors ---
pub mod CreateTerminal;
