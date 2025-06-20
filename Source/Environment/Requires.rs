//! # Requires Trait
//!
//! Defines the `Requires` trait, which is the heart of the capability-based
//! dependency injection system.

use std::sync::Arc;

use super::Environment::Environment;

/// A trait that enables an environment (`Self`) to provide a specific
/// capability (`Capability`).
///
/// This is the central mechanism for dependency injection. An `ActionEffect`
/// can be generic over an environment `TEnvironment` as long as it has the
/// bound `TEnvironment: Requires<MyCapability>`. The effect can then call
/// `Environment.Require()` to receive a shared instance of the service it needs
/// to perform its operation.
///
/// The `Capability` is typically a trait object, such as
/// `dyn FileSystemReader`.
pub trait Requires<Capability:?Sized>: Environment {
	/// Returns the required capability from the environment, wrapped in an
	/// `Arc` for safe, shared ownership.
	fn Require(&self) -> Arc<Capability>;
}

/// A blanket implementation that allows an `Arc<TEnvironment>` to provide a
/// capability if the inner environment `TEnvironment` can provide it.
///
/// This is a crucial piece of ergonomics that allows code to call `.Require()`
/// directly on a shared `Arc<TEnvironment>` reference without needing to
/// dereference it first, simplifying effect implementation logic.
impl<TEnvironment:Requires<Capability> + ?Sized, Capability:?Sized> Requires<Capability> for Arc<TEnvironment> {
	/// Fulfills the requirement by dereferencing the `Arc` and calling the
	/// `Require` method on the inner environment `TEnvironment`.
	fn Require(&self) -> Arc<Capability> { (**self).Require() }
}
