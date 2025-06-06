// Land_Common/src/runtime.rs

use std::{future::Future, pin::Pin, sync::Arc};

use crate::effect::ActionEffect;
// Assuming Environment and Requires are defined in crate::environment
use crate::environment::{Environment, Requires};
// CommonError might be used as a default error type in some generic contexts,
// though specific effects usually define their own error type.
use crate::errors::CommonError;

/// Generic trait for a runtime capable of executing `ActionEffect`s.
///
/// A runtime is responsible for providing the necessary context (environment)
/// to an effect so that the effect can perform its operations.
///
/// # Type Parameters
/// * `ContextEnv`: The primary environment type managed by this runtime. This
///   environment is expected to be able to provide the specific capabilities
///   (via the `Requires` trait) needed by the effects being run.
#[async_trait::async_trait]
pub trait Runtime<ContextEnv:Environment>: Send + Sync {
	/// Retrieves the main environment associated with this runtime.
	///
	/// This environment (`ContextEnv`) is the one that implements
	/// `Requires<Capability>` for the various provider traits (e.g.,
	/// `FsReader`, `ConfigProvider`).
	fn get_environment(&self) -> Arc<ContextEnv>;

	/// Executes an `ActionEffect`.
	///
	/// The core responsibility of this method is to take an `ActionEffect` and
	/// provide it with the appropriate environment accessor (`E_Accessor`) it
	/// expects.
	///
	/// # Type Parameters
	/// * `E_Accessor`: The type of the environment accessor that the
	///   `ActionEffect`'s closure expects. In many practical scenarios (like
	///   the Mountain architecture), this is an `Arc` to a concrete runtime
	///   struct (e.g., `Arc<AppRuntime>`) which itself provides access to the
	///   underlying `ContextEnv`.
	/// * `Err`: The error type that the effect's operation can return.
	/// * `Out`: The success output type of the effect's operation.
	///
	/// # Constraints
	/// * `E_Accessor: Environment + Send + Sync + 'static`: The accessor itself
	///   must be a valid environment type. If `E_Accessor` is `Arc<T>`, then
	///   `T` should implement `Environment`.
	/// * `Err: Send + Sync + 'static`: The error type must be sendable across
	///   threads.
	/// * `Out: Send + Sync + 'static`: The output type must be sendable across
	///   threads.
	/// * `ContextEnv: Requires<E_Accessor>`: This is a crucial constraint. It
	///   means that the runtime's main environment (`ContextEnv`) must be able
	///   to provide (or be convertible to) the specific accessor type
	///   `E_Accessor` that the effect's closure needs.
	///   - If an effect is `ActionEffect<Arc<AppRuntime>, _, _>`, and
	///     `ContextEnv` is `MountainEnvironment`, then `MountainEnvironment`
	///     would need to implement `Requires<Arc<AppRuntime>>`. This often
	///     means `MountainEnvironment` needs a way to get an `Arc` to its
	///     containing `AppRuntime`, which can be complex for a generic
	///     `Runtime` trait implementation.
	///   - More commonly, `AppRuntime` itself might implement
	///     `Runtime<MountainEnvironment>` and have a specialized `run` method:
	///     `async fn run<Err, Out>(&self, effect: ActionEffect<Arc<Self>, Err,
	///     Out>)`.
	async fn run<E_Accessor, Err, Out>(&self, effect:ActionEffect<E_Accessor, Err, Out>) -> Result<Out, Err>
	where
		E_Accessor: Environment + Send + Sync + 'static,
		Err: Send + Sync + 'static,
		Out: Send + Sync + 'static,
		ContextEnv: Requires<E_Accessor>; // ContextEnv must be able to provide E_Accessor
}

/// A default, simplified runtime implementation.
///
/// This struct is primarily for conceptual illustration or very simple use
/// cases. In a full application like Mountain, a more specialized `AppRuntime`
/// struct would likely be defined. That `AppRuntime` might either:
/// 1. Implement the `Runtime<MountainEnvironment>` trait directly.
/// 2. Or, more commonly, have its own `run` method specifically typed for
///    effects expecting `Arc<AppRuntime>` as their accessor, like: `async fn
///    run<Err, Out>(&self, effect: ActionEffect<Arc<Self>, Err, Out>) ->
///    Result<Out, Err>`.
///
/// This `DefaultRuntime` demonstrates how the generic `Runtime` trait could be
/// satisfied, but it highlights the challenge with the `ContextEnv:
/// Requires<E_Accessor>` constraint when `E_Accessor` is an `Arc` to a
/// containing runtime.
pub struct DefaultRuntime<ContextEnv:Environment> {
	environment:Arc<ContextEnv>,
}

impl<ContextEnv:Environment> DefaultRuntime<ContextEnv> {
	/// Creates a new `DefaultRuntime` with the given environment.
	pub fn new(environment:Arc<ContextEnv>) -> Self { Self { environment } }
}

#[async_trait::async_trait]
impl<ContextEnv:Environment + Send + Sync + 'static> Runtime<ContextEnv> for DefaultRuntime<ContextEnv> {
	fn get_environment(&self) -> Arc<ContextEnv> { Arc::clone(&self.environment) }

	async fn run<E_Accessor, Err, Out>(&self, effect:ActionEffect<E_Accessor, Err, Out>) -> Result<Out, Err>
	where
		E_Accessor: Environment + Send + Sync + 'static,
		Err: Send + Sync + 'static,
		Out: Send + Sync + 'static,
		ContextEnv: Requires<E_Accessor>, // The environment held by DefaultRuntime must provide E_Accessor
	{
		// The conceptual flow for this generic `run` is:
		// 1. Get the required accessor `E_Accessor` from `self.environment` (which is
		//    of type `ContextEnv`). This relies on `ContextEnv` implementing
		//    `Requires<E_Accessor>`.
		// 2. Pass this `required_accessor` to the effect's function.

		// let required_accessor: E_Accessor = self.environment.require();
		// effect.apply(required_accessor).await // Effect::apply takes E_Accessor

		// The following panic message highlights a key architectural point:
		// Most effects in the Mountain system are defined as:
		//   `ActionEffect<Arc<AppRuntimeConcrete>, ErrorType, OutputType>`
		// Their internal closures expect `Arc<AppRuntimeConcrete>` as the argument.
		//
		// The most straightforward way to run such an effect is by having a method
		// directly on `AppRuntimeConcrete` like:
		//   `impl AppRuntimeConcrete {`
		//   `  async fn run(self: Arc<Self>, effect: ActionEffect<Arc<Self>, E, O>) ->
		// Result<O, E> {`   `    effect.apply(self).await`
		//   `  }`
		//   `}`
		//
		// This `DefaultRuntime::run` is too generic to easily fulfill that pattern
		// without `ContextEnv` (e.g., `MountainEnvironment`) having a reference back
		// to its containing `AppRuntimeConcrete` to satisfy
		// `Requires<Arc<AppRuntimeConcrete>>`. Such a circular dependency is often
		// avoided.
		//
		// Therefore, `DefaultRuntime` serves more as a demonstration of the `Runtime`
		// trait's generic nature rather than a production-ready executor for all
		// effect patterns.
		panic!(
			"DefaultRuntime::run is a conceptual implementation. For ActionEffect specifically designed to take \
			 Arc<SpecificAppRuntime> as their accessor, it's typical for SpecificAppRuntime itself to have a method \
			 like `async fn run(self: Arc<Self>, effect: ActionEffect<Arc<Self>, Err, Out>) -> Result<Out, Err>` \
			 which directly calls `effect.apply(self).await`."
		);
	}
}

// Notes on expected Environment and Requires trait definitions (typically in
// `crate::environment`):
//
// /// `crate::environment::Environment`
// /// A base marker trait for any environment or context provider.
// /// Must be Send, Sync, and 'static to be easily shareable.
// pub trait Environment: Send + Sync + 'static {}
//
// /// Blanket implementation to allow `Arc<T>` to be treated as an
// `Environment` /// if `T` itself implements `Environment`. This is useful if
// an `Arc<ActualEnvironment>` /// is passed around as the environment accessor.
// impl<T: Environment + ?Sized> Environment for Arc<T> {}
//
// /// `crate::environment::Requires<T>`
// /// A trait indicating that an `Environment` can provide a specific
// capability `T`. /// `T` is often an `Arc<dyn TraitName + Send + Sync>`.
// pub trait Requires<Capability: ?Sized>: Environment {
//     /// Retrieves the required capability.
//     fn require(&self) -> Arc<Capability>;
// }
//
// /// Blanket implementation allowing an `Arc<Env>` to provide capabilities if
// `Env` does. /// This helps when the primary environment handle is an `Arc`.
// impl<EnvType: Requires<Capability> + ?Sized, Capability: ?Sized>
// Requires<Capability> for Arc<EnvType> {     fn require(&self) ->
// Arc<Capability> {         (**self).require() // Dereference Arc and call
// require on the inner EnvType     }
// }
