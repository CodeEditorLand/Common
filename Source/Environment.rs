// Land_Common/src/environment.rs
use std::sync::Arc;

/// Base marker trait for any environment that can run effects.
/// Specific effects will require sub-traits like `FsReader`, `ConfigProvider`,
/// etc. Environments are expected to be `Send + Sync + 'static` to be easily
/// shareable across threads and have a known lifetime.
pub trait Environment: Send + Sync + 'static {}

/// Trait to indicate that an environment can provide a specific capability `T`.
/// `T` will typically be an `Arc` to a dyn Trait object, like `Arc<dyn FsReader
/// + Send + Sync>`.
///
/// This trait is bounded by `Environment`, meaning only types that are
/// themselves `Environment`s can provide capabilities.
pub trait Requires<T:?Sized>: Environment {
	/// Retrieves the required capability `T`.
	fn require(&self) -> Arc<T>;
}

// Blanket implementation to allow Arc<Env> to be an Environment if Env is.
// This is useful if the AppRuntime itself holds an Arc<ActualEnvironment> and
// AppRuntime needs to be Environment.
impl<T:Environment + ?Sized> Environment for Arc<T> {}

// Blanket implementation to allow Arc<Env> to provide capabilities if Env does.
// This helps in situations where an Arc<ActualEnvironment> is passed around.
impl<T:Requires<Cap> + ?Sized, Cap:?Sized> Requires<Cap> for Arc<T> {
	fn require(&self) -> Arc<Cap> { (**self).require() }
}
