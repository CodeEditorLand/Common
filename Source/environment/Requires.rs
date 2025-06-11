// Defines the `Requires` trait, the core of the dependency injection system.

use std::sync::Arc;

use super::Environment;

/// A trait that enables an environment (`Self`) to provide a specific
/// capability (`Capability`).
///
/// This is the central mechanism for dependency injection. An `ActionEffect`
/// can be generic over an environment `E` as long as it has the bound
/// `E: Requires<MyCapability>`. The effect can then call
/// `Environment.Require()` to get a shared instance of the service it needs.
///
/// `Capability` will typically be an `Arc<dyn ServiceTrait>`.
pub trait Requires<Capability:?Sized>: Environment {
	/// Returns the required capability, wrapped in an `Arc` for shared
	/// ownership.
	fn Require(&self) -> Arc<Capability>;
}

/// A blanket implementation that allows an `Arc<E>` to provide a capability if
/// the inner environment `E` can provide it.
///
/// This is a crucial piece of ergonomics, as it allows us to pass around an
/// `Arc<Environment>` and call `.Require()` on it directly, without needing to
/// dereference it first.
impl<E:Requires<Capability> + ?Sized, Capability:?Sized> Requires<Capability> for Arc<E> {
	/// Dereferences the `Arc` to call the `Require` method on the inner
	/// environment `E`.
	fn Require(&self) -> Arc<Capability> { (**self).Require() }
}
