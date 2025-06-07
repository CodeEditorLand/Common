// Land_Common/src/effect.rs
use std::{future::Future, pin::Pin, sync::Arc};

use crate::environment::Environment; // Base marker trait for environments
// AppRuntime is expected to be the concrete runtime accessor passed to effects.
// It should provide a way to get the actual environment that implements provider traits.
// E.g., AppRuntime would have a method like `get_environment()` which returns `Arc<MountainEnvironment>`.
// Then, `MountainEnvironment` would implement `Requires<Arc<dyn FsReader>>`, etc.
use crate::runtime::AppRuntime; // This should be the actual type from your runtime module.

/// Represents an action that can be performed within the application,
/// typically involving interaction with the environment (e.g., filesystem,
/// configuration, IPC).
///
/// # Type Parameters
/// * `E_Accessor`: The type of the environment accessor that the effect's
///   closure expects. For effects designed to work with the Mountain
///   architecture, this is typically `Arc<AppRuntime>`. The `AppRuntime` then
///   provides access to the concrete environment (e.g., `MountainEnvironment`)
///   which implements specific provider traits (like `FsReader`,
///   `ConfigProvider`).
/// * `Err`: The error type that the effect's operation can return.
/// * `Out`: The success output type of the effect's operation.
pub struct ActionEffect<E_Accessor:?Sized, Err, Out> {
	// The core logic of the effect.
	// It's a boxed, pinned future that takes an accessor to the environment/runtime
	// and produces a Result<Out, Err>.
	// The accessor (E_Accessor, typically Arc<AppRuntime>) is used to retrieve
	// specific capabilities (e.g., Arc<dyn FsReader>) from the environment.
	func: Arc<
		dyn Fn(
				E_Accessor, // The type of the environment accessor passed in.
			) -> Pin<Box<dyn Future<Output = Result<Out, Err>> + Send>>
			+ Send
			+ Sync,
	>,
}

impl<E_Accessor:?Sized, Err, Out> ActionEffect<E_Accessor, Err, Out> {
	/// Creates a new `ActionEffect`.
	///
	/// # Argument
	/// * `func`: A closure that takes an environment accessor (`E_Accessor`)
	///   and returns a pinned, boxed future. This future represents the
	///   asynchronous operation of the effect.
	pub fn new(
		func:Arc<dyn Fn(E_Accessor) -> Pin<Box<dyn Future<Output = Result<Out, Err>> + Send>> + Send + Sync>,
	) -> Self {
		Self { func }
	}

	/// Applies the effect's function to the given environment accessor.
	///
	/// This method is typically called by a `Runtime` implementation (e.g.,
	/// `AppRuntime::run`). The `Runtime` is responsible for providing the
	/// correct `env_accessor`.
	///
	/// # Argument
	/// * `env_accessor`: The environment accessor (e.g., `Arc<AppRuntime>`)
	///   required by the effect's closure.
	///
	/// # Returns
	/// A future that resolves to the result of the effect's operation.
	pub async fn apply(&self, env_accessor:E_Accessor) -> Result<Out, Err>
	where
		E_Accessor: Clone, /* Require Clone if the accessor needs to be cloned into the future
		                    * This bound might not always be necessary if `func` takes `E_Accessor` by value
		                    * and moves it, but often `Arc<T>` is used which is Clone. */ {
		(self.func)(env_accessor).await
	}
}

// Implement Clone for ActionEffect.
// This allows effects to be cloned if they need to be stored or passed around,
// as the underlying `func` is an Arc-wrapped trait object.
impl<E_Accessor:?Sized, Err, Out> Clone for ActionEffect<E_Accessor, Err, Out> {
	fn clone(&self) -> Self { ActionEffect { func:Arc::clone(&self.func) } }
}

// Example of how an effect would be constructed in, for instance,
// `fs_effects.rs`:
//
// use crate::effect::ActionEffect;
// use crate::runtime::AppRuntime; // Your specific AppRuntime type
// use crate::environment::{Environment, Requires}; // Your Environment &
// Requires traits use crate::fs_effects::FsReader; // The specific capability
// trait use crate::errors::CommonError;
// use std::path::PathBuf;
// use std::sync::Arc;
//
// pub fn read_file_effect(path: PathBuf) -> ActionEffect<Arc<AppRuntime>,
// CommonError, Vec<u8>> { ActionEffect::new(Arc::new(move
// |app_runtime_accessor: Arc<AppRuntime>| { let p_clone = path.clone();
// Box::pin(async move {
// // 1. Get the concrete environment (e.g., MountainEnvironment) from
// AppRuntime let concrete_env = app_runtime_accessor.get_environment(); //
// Assuming AppRuntime has this method
//
// // 2. Require the FsReader capability from the concrete environment
// let fs_reader_capability: Arc<dyn FsReader + Send + Sync> =
// concrete_env.require();
//
// // 3. Use the capability
// fs_reader_capability.read_file(&p_clone).await
// })
// }))
// }
//
// Where AppRuntime might be:
// pub struct AppRuntime {
//     // actual_environment: Arc<MountainEnvironment>,
//     // ... other runtime resources ...
// }
// impl AppRuntime {
//     // pub fn get_environment(&self) -> Arc<MountainEnvironment> {
// self.actual_environment.clone() } }
// impl Environment for AppRuntime {} // If AppRuntime itself acts as an
// Environment accessor
//
// And MountainEnvironment would implement:
// impl Requires<Arc<dyn FsReader + Send + Sync>> for MountainEnvironment {
//     fn require(&self) -> Arc<dyn FsReader + Send + Sync> {
// Arc::new(self.clone()) /* if it impls FsReader */ } }
