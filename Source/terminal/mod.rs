

//
// @module terminal
// @description This module defines the abstract contract for the integrated Terminal service.
// It includes the `TerminalProvider` trait and the `ActionEffect` constructors
// for every terminal-related operation.
//

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod TerminalProvider;
pub use self::TerminalProvider::TerminalProvider;

// --- Effect Constructors ---
mod CreateTerminal;
mod DisposeTerminal;
mod SendTextToTerminal;

pub use self::CreateTerminal::CreateTerminal;
pub use self::DisposeTerminal::DisposeTerminal;
pub use self::SendTextToTerminal::SendTextToTerminal;
