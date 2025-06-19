//! # HasEnvironment Trait
//!
//! Defines the `HasEnvironment` trait for any context-holding object that
//! contains and provides access to an `Environment`.

use std::sync::Arc;

use super::Environment::Environment;

/// A generic trait for any type that holds a shared, reference-counted pointer
/// to an `Environment`.
///
/// This is primarily implemented by the `ApplicationRunTime`, which needs to
/// provide access to the `MountainEnvironment` it manages so that
/// `ActionEffect`s can be executed with the correct context and capabilities.
pub trait HasEnvironment {
	/// The specific, concrete type of the `Environment` this struct holds.
	type EnvironmentType: Environment;

	/// Gets a shared, reference-counted pointer to the environment.
	fn GetEnvironment(&self) -> Arc<Self::EnvironmentType>;
}
