// Defines the core `Environment` marker trait, the foundation of the
// application's dependency injection system.

use std::sync::Arc;

/// A marker trait for any struct that represents an application's environment.
///
/// An `Environment` holds all the concrete service implementations and
/// application state needed to execute `ActionEffect`s. By requiring that all
/// environments implement this trait, we can write generic code that operates
/// on any valid environment.
///
/// The trait bounds `Send + Sync + 'static` are critical to ensure that the
/// environment can be safely shared across asynchronous tasks and threads.
pub trait Environment: Send + Sync + 'static {}

/// A blanket implementation that allows a shared, reference-counted pointer to
/// an `Environment` (`Arc<T>`) to also be treated as an `Environment`.
///
/// This is a key piece of ergonomics that enables the DI system to work
/// seamlessly with shared state.
impl<T:Environment + ?Sized> Environment for Arc<T> {}
