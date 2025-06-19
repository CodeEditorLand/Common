//! # Terminal Service
//!
//! This module defines the abstract contract for the integrated Terminal
//! service. It includes the `TerminalProvider` trait and the `ActionEffect`
//! constructors for every terminal-related operation.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod TerminalProvider;
// pub use self::TerminalProvider::TerminalProvider;

// --- Effect Constructors ---
pub mod CreateTerminal;
// Additional effects like SendTextToTerminal, DisposeTerminal would go here.

// pub use self::CreateTerminal::CreateTerminal;
