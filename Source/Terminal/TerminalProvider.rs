// File: Common/Source/Terminal/TerminalProvider.rs
// Role: Defines the abstract service trait for creating and managing integrated
// terminal instances. Responsibilities:
//   - Provide a contract for creating, showing, hiding, and disposing of
//     terminals.
//   - Provide a contract for sending input to terminals and querying their
//     state.

//! # TerminalProvider Trait
//!
//! Defines the abstract service trait for creating and managing integrated
//! terminal instances.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// integrated terminal processes.
/// Implemented by `MountainEnvironment` and responsible for spawning and
/// managing native pseudo-terminal (PTY) processes, handling their I/O, and
/// managing their lifecycle.
#[async_trait]
pub trait TerminalProvider: Environment + Send + Sync {
	/// Creates a new terminal instance with the given options.
	///
	/// # Parameters
	/// * `OptionsValue`: A `serde_json::Value` DTO representing
	///   `TerminalOptions`, which can specify the name, shell path, arguments,

	///   etc.
	///
	/// # Returns
	/// A `Result` containing a JSON `Value` with details of the created
	/// terminal (e.g., its ID and process ID), or a `CommonError` on failure.
	async fn CreateTerminal(&self, OptionsValue:Value) -> Result<Value, CommonError>;

	/// Sends a string of text as input to a specific terminal instance's
	/// underlying pseudo-terminal process.
	///
	/// # Parameters
	/// * `TerminalId`: The unique identifier of the target terminal.
	/// * `Text`: The text to send to the terminal's standard input.
	async fn SendTextToTerminal(&self, TerminalId:u64, Text:String) -> Result<(), CommonError>;

	/// Disposes of a specific terminal instance. This involves terminating the
	/// underlying shell process and cleaning up any associated resources.
	///
	/// # Parameters
	/// * `TerminalId`: The unique identifier of the terminal to dispose of.
	async fn DisposeTerminal(&self, TerminalId:u64) -> Result<(), CommonError>;

	/// Shows a terminal in the UI, optionally giving it focus.
	///
	/// # Parameters
	/// * `TerminalId`: The unique identifier of the terminal to show.
	/// * `PreserveFocus`: If `true`, the terminal panel is revealed but focus
	///   is not given to it.
	async fn ShowTerminal(&self, TerminalId:u64, PreserveFocus:bool) -> Result<(), CommonError>;

	/// Hides the terminal panel if the specified terminal is active.
	///
	/// # Parameters
	/// * `TerminalId`: The unique identifier of the terminal to hide.
	async fn HideTerminal(&self, TerminalId:u64) -> Result<(), CommonError>;

	/// Gets the process ID (PID) of the underlying shell process for a
	/// terminal.
	///
	/// # Parameters
	/// * `TerminalId`: The unique identifier of the terminal to query.
	/// # Returns
	/// An `Option<u32>` with the PID if the process is running, or `None`.
	async fn GetTerminalProcessId(&self, TerminalId:u64) -> Result<Option<u32>, CommonError>;

	/// Resizes the PTY backing a terminal to the given column/row count. The
	/// shell process receives SIGWINCH (POSIX) or the equivalent on Windows,
	/// so line-editing utilities like readline/shells pick up the new size.
	///
	/// # Parameters
	/// * `TerminalId`: The unique identifier of the terminal to resize.
	/// * `Columns`:   Desired column count (≥ 1).
	/// * `Rows`:      Desired row count (≥ 1).
	async fn ResizeTerminal(&self, TerminalId:u64, Columns:u16, Rows:u16) -> Result<(), CommonError>;
}
