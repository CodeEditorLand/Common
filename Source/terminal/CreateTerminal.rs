use std::sync::Arc;

use serde_json::Value;

/// @module CreateTerminal
/// @description Defines the ActionEffect for creating a new integrated terminal
/// instance.
use super::TerminalProvider::TerminalProvider;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

/// Creates an effect that, when executed, will create a new terminal instance
/// based on the provided options.
///
/// The effect will use the `TerminalProvider` capability from the environment
/// to perform the actual creation, which typically involves spawning a new
/// pseudo-terminal (PTY) process.
///
/// @param OptionsValue - A `serde_json::Value` representing the
/// `TerminalOptions` DTO,   containing properties like the name, shell path,
/// and arguments for the terminal.
///
/// @returns An `ActionEffect` that resolves with a JSON `Value` containing
/// details   of the newly created terminal, such as its ID and process ID
/// (PID).
pub fn CreateTerminal<Runtime>(OptionsValue:Value) -> ActionEffect<Arc<Runtime>, CommonError, Value>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn TerminalProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let OptionsClone = OptionsValue.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn TerminalProvider> = Environment.Require();
			Provider.CreateTerminal(OptionsClone).await
		})
	}))
}
