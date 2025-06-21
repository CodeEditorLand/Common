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

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod TerminalProvider;
// pub use self::TerminalProvider::TerminalProvider;

// --- Effect Constructors ---
pub mod CreateTerminal;
// pub mod DisposeTerminal;
// pub mod GetTerminalProcessId;
// pub mod HideTerminal;
// pub mod SendTextToTerminal;
// pub mod ShowTerminal;

// pub use self::{
// 	CreateTerminal::CreateTerminal,
// 	DisposeTerminal::DisposeTerminal,
// 	GetTerminalProcessId::GetTerminalProcessId,
// 	HideTerminal::HideTerminal,
// 	SendTextToTerminal::SendTextToTerminal,
// 	ShowTerminal::ShowTerminal,
// };
