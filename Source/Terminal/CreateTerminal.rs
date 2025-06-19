//! # CreateTerminal Effect
//!
//! Defines the `ActionEffect` for creating a new integrated terminal instance.

use std::sync::Arc;

use serde_json::Value;

use super::TerminalProvider::TerminalProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will create a new terminal instance
/// based on the provided options.
///
/// The effect will use the `TerminalProvider` capability from the environment
/// to perform the actual creation, which typically involves spawning a new
/// pseudo-terminal (PTY) process.
///
/// # Parameters
/// * `OptionsValue`: A `serde_json::Value` representing the `TerminalOptions`
///   DTO, containing properties like the name, shell path, and arguments for
///   the terminal.
///
/// # Returns
/// An `ActionEffect` that resolves with a JSON `Value` containing details of
/// the newly created terminal, such as its ID and process ID (PID).
pub fn CreateTerminal<TRunTime>(OptionsValue:Value) -> ActionEffect<Arc<TRunTime>, CommonError, Value>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn TerminalProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let OptionsClone = OptionsValue.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Provider:Arc<dyn TerminalProvider> = Environment.Require();
			Provider.CreateTerminal(OptionsClone).await
		})
	}))
}
