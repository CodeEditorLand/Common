//! # TerminalProvider Trait
//!
//! Defines the abstract service trait for creating and managing integrated
//! terminal instances.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// integrated terminal processes.
///
/// This trait is implemented by `MountainEnvironment`, and its methods are
/// responsible for spawning and managing native pseudo-terminal (PTY)
/// processes, handling their I/O, and managing their lifecycle.
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
}
