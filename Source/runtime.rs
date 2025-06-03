// ORIGIN INFORMATION:
// This code block was extracted by a script.
// Source Markdown File: Backup/LCMSCR/Document/252_MODEL.md
// Source Block Index in MD (Overall): 9
// Original Fence Info String: (empty)
// Content SHA256 (of this block):
// 92a0a25a2e7ac602d848de11b2072b65dcc379662f4e7ef222139b3831366c23 Extracted to
// File: Backup/LCMSCR/Code/Land_Common/src/runtime.rs Extraction Timestamp:
// 2025-06-03T21:43:49.466Z --- END OF ORIGIN INFORMATION ---

// Land_Common/src/runtime.rs
use std::{future::Future, pin::Pin, sync::Arc};

use crate::effect::ActionEffect;
use crate::environment::{Environment, Requires}; // For trait bounds
use crate::errors::CommonError; // For default error type if needed (not actively used here)

/// Generic trait for a runtime capable of executing ActionEffects.
///
/// `ContextEnv` is the primary environment type managed by this runtime.
#[async_trait::async_trait]
pub trait Runtime<ContextEnv:Environment>: Send + Sync {
	/// Retrieves the main environment associated with this runtime.
	fn get_environment(&self) -> Arc<ContextEnv>;

	/// Executes an `ActionEffect`.
	///
	/// # Type Parameters
	/// * `E_Accessor`: The type of the environment accessor that the
	///   `ActionEffect`'s closure expects. Often, this is an `Arc` to a
	///   specific runtime or environment struct (e.g., `Arc<AppRuntime>`).
	/// * `Err`: The error type the effect can return.
	/// * `Out`: The output type the effect can return on success.
	///
	/// # Constraints
	/// * `E_Accessor: Environment + Send + Sync + 'static`: The accessor itself
	///   (or the type it points to) must conform to the `Environment` trait. If
	///   `E_Accessor` is `Arc<T>`, then `T` should implement `Environment`, and
	///   there might be a blanket `impl<T: Environment> Environment for
	///   Arc<T>`.
	/// * `ContextEnv: Requires<E_Accessor>`: The runtime's main environment
	///   (`ContextEnv`) must be able to provide the specific accessor type
	///   `E_Accessor` required by the effect. This is key for the runtime to
	///   supply the correct context to the effect.
	async fn run<E_Accessor, Err, Out>(&self, effect:ActionEffect<E_Accessor, Err, Out>) -> Result<Out, Err>
	where
		E_Accessor: Environment + Send + Sync + 'static,
		Err: Send + Sync + 'static,
		Out: Send + Sync + 'static,
		ContextEnv: Requires<E_Accessor>;
}

/// A default, simplified runtime implementation.
///
/// This implementation is primarily conceptual. In a full application like
/// Mountain, a more specialized `AppRuntime` would likely exist, which might
/// either wrap a similar structure or implement the `Runtime` trait directly
/// tailored to its needs.
pub struct DefaultRuntime<ContextEnv:Environment> {
	environment:Arc<ContextEnv>,
}

impl<ContextEnv:Environment> DefaultRuntime<ContextEnv> {
	pub fn new(environment:Arc<ContextEnv>) -> Self { Self { environment } }
}

// This makes DefaultRuntime implement the generic Runtime trait.
#[async_trait::async_trait]
impl<ContextEnv:Environment + Send + Sync + 'static> Runtime<ContextEnv> for DefaultRuntime<ContextEnv> {
	fn get_environment(&self) -> Arc<ContextEnv> { Arc::clone(&self.environment) }

	async fn run<E_Accessor, Err, Out>(&self, effect:ActionEffect<E_Accessor, Err, Out>) -> Result<Out, Err>
	where
		E_Accessor: Environment + Send + Sync + 'static,
		Err: Send + Sync + 'static,
		Out: Send + Sync + 'static,
		ContextEnv: Requires<E_Accessor>, {
		// The primary pattern for effects defined in `language_feature_effects.rs` is
		// `ActionEffect<Arc<AppRuntimeConcrete>, _, _>`. The closure inside such an
		// effect expects an `Arc<AppRuntimeConcrete>` as its argument.
		//
		// An `AppRuntimeConcrete` would typically run such an effect by calling:
		// `(effect.func)(self_arc_app_runtime_concrete).await`
		// (where `self_arc_app_runtime_concrete` is an `Arc` of the
		// `AppRuntimeConcrete` instance).
		//
		// This generic `DefaultRuntime::run` method tries to abstract that.
		// It would fetch the required accessor `E_Accessor` using
		// `self.environment.require()`, and then pass it to
		// `(effect.func)(accessor).await`.
		//
		// However, if `E_Accessor` is `Arc<AppRuntimeConcrete>` and `ContextEnv` is,
		// for example, `AppEnvironment` (the environment *inside*
		// `AppRuntimeConcrete`), the constraint `AppEnvironment:
		// Requires<Arc<AppRuntimeConcrete>>` implies that `AppEnvironment`
		// must be able to provide an `Arc` to its containing `AppRuntimeConcrete`.
		// This can create a circular dependency or require `AppEnvironment` to be aware
		// of its container, which might be complex for a truly generic
		// `DefaultRuntime`.
		//
		// Therefore, this `DefaultRuntime` is more of a placeholder or a template.
		// The actual execution of effects expecting `Arc<AppRuntimeConcrete>` is
		// usually handled by a method on `AppRuntimeConcrete` itself.
		let required_accessor:E_Accessor = self.environment.require();
		// (effect.func)(required_accessor).await // or
		// effect.apply(required_accessor).await

		// The original panic is kept to emphasize that for the common AppRuntime
		// pattern, AppRuntime.run() is the typical executor. This DefaultRuntime
		// would need careful instantiation to work seamlessly with effects expecting
		// Arc<AppRuntime>.
		panic!(
			"DefaultRuntime::run is a conceptual implementation. For ActionEffects expecting Arc<SpecificAppRuntime>, \
			 SpecificAppRuntime::run(self: Arc<Self>, effect) is typically used."
		);
	}
}

// Note on Environment and Requires traits (expected to be in
// crate::environment):
//
// /// `crate::environment::Environment`
// /// A marker trait for environments or context providers.
// pub trait Environment: Send + Sync {}
//
// /// Blanket implementation to allow `Arc<T>` to be treated as `Environment`
// if `T` is. /// This helps satisfy `E_Accessor: Environment` when `E_Accessor`
// is `Arc<ActualEnv>`. impl<T: Environment + ?Sized> Environment for Arc<T> {}
//
// /// `crate::environment::Requires<T>`
// /// Trait for an entity (usually an Environment) that can provide a specific
// capability `T`. pub trait Requires<T>: Environment { // `Requires<T>` implies
// `Environment`     fn require(&self) -> T;
// }
