//! # GetAllCommands Effect
//!
//! Defines the `ActionEffect` for retrieving all registered command
//! identifiers.

use std::sync::Arc;

use super::CommandExecutor::CommandExecutor;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will retrieve a list of all currently
/// registered command identifiers.
///
/// This includes both native commands implemented in Rust and proxied commands
/// contributed by external sidecars. It uses the `CommandExecutor` capability
/// from the environment to perform the operation.
///
/// # Returns
///
/// An `ActionEffect` that resolves with a `Vec<String>` of command
/// identifiers.
pub fn GetAllCommands<TRunTime>() -> ActionEffect<Arc<TRunTime>, CommonError, Vec<String>>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn CommandExecutor>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Executor:Arc<dyn CommandExecutor> = Environment.Require();
			Executor.GetAllCommands().await
		})
	}))
}
