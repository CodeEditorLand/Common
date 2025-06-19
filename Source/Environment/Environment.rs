//! # Environment Trait
//!
//! Defines the core `Environment` marker trait, which serves as the foundation
//! of the application's dependency injection and capability system.

use std::sync::Arc;

/// A marker trait for any struct that represents an application's environment.
///
/// An `Environment` is a container that holds all the concrete service
/// implementations and application state necessary for executing
/// `ActionEffect`s. By requiring that all environments implement this trait,
/// the system can be generic over any valid environment context.
///
/// The `Send + Sync + 'static` bounds are critical, ensuring that the
/// environment can be safely shared across asynchronous tasks and threads,
/// which is essential for a concurrent application.
pub trait Environment: Send + Sync + 'static {}

/// A blanket implementation that allows a shared, reference-counted pointer to
/// an `Environment` (`Arc<TEnvironment>`) to also be treated as an
/// `Environment`.
///
/// This is a key ergonomic feature of the dependency injection system, enabling
/// shared state to be passed around and used seamlessly without needing to
/// constantly dereference `Arc` pointers.
impl<TEnvironment:Environment + ?Sized> Environment for Arc<TEnvironment> {}
