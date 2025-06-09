// File: Common/Source/CommandEffect.rs
// Responsibility: Responsibility could not be determined.
// Modified: 2025-06-04 01:10:48 UTC

// Land_Common/src/command_effects.rs
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;

// Ensure AppRuntime is the correct type from your runtime module.
// This accessor is used by effects to get to the concrete environment.
use crate::runtime::AppRuntime;
use crate::{
	effect::ActionEffect,
	environment::{Environment, Requires},
	errors::CommonError,
};

/// Trait for executing and managing commands within the application.
///
/// An environment implementing this trait can execute predefined or dynamically
/// registered commands, often involving IPC with sidecars or native handlers.
#[async_trait]
pub trait CommandExecutor: Environment {
	/// Executes a command with the given ID and arguments.
	async fn execute_command(&self, command_id:String, args:Value) -> Result<Value, CommonError>;

	/// Registers a command that can be executed, typically associating it with
	/// a sidecar.
	async fn register_command(&self, sidecar_id:String, command_id:String) -> Result<(), CommonError>;

	/// Unregisters a previously registered command.
	async fn unregister_command(&self, sidecar_id:String, command_id:String) -> Result<(), CommonError>;

	/// Retrieves a list of all currently registered command IDs.
	async fn get_all_commands(&self) -> Result<Vec<String>, CommonError>;
}

// --- Effect Constructors ---

/// Creates an effect to execute a command.
///
/// The effect, when run, will use the `CommandExecutor` capability of the
/// environment to dispatch the command.
pub fn execute_command(command_id:String, args:Value) -> ActionEffect<Arc<AppRuntime>, CommonError, Value> {
	// Effect expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let cid_clone = command_id.clone();
		let args_clone = args.clone();
		Box::pin(async move {
			// Get the concrete environment (e.g., MountainEnvironment)
			let concrete_env = app_runtime_accessor.get_environment();
			// Require the CommandExecutor capability
			let executor:Arc<dyn CommandExecutor + Send + Sync> = concrete_env.require();
			executor.execute_command(cid_clone, args_clone).await
		})
	}))
}

/// Creates an effect to register a command.
pub fn register_command(sidecar_id:String, command_id:String) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Effect expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let sid_clone = sidecar_id.clone();
		let cid_clone = command_id.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let executor:Arc<dyn CommandExecutor + Send + Sync> = concrete_env.require();
			executor.register_command(sid_clone, cid_clone).await
		})
	}))
}

/// Creates an effect to unregister a command.
pub fn unregister_command(sidecar_id:String, command_id:String) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Effect expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let sid_clone = sidecar_id.clone();
		let cid_clone = command_id.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let executor:Arc<dyn CommandExecutor + Send + Sync> = concrete_env.require();
			executor.unregister_command(sid_clone, cid_clone).await
		})
	}))
}

/// Creates an effect to get all registered command IDs.
pub fn get_all_commands() -> ActionEffect<Arc<AppRuntime>, CommonError, Vec<String>> {
	// Effect expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let executor:Arc<dyn CommandExecutor + Send + Sync> = concrete_env.require();
			executor.get_all_commands().await
		})
	}))
}
