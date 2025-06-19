//! # ApplicationRunTime Trait
//!
//! Defines the `ApplicationRunTime` trait, which specifies the contract for any
//! "engine" responsible for executing `ActionEffect`s.

use std::sync::Arc;

use async_trait::async_trait;

use super::ActionEffect::ActionEffect;
use crate::Environment::{HasEnvironment::HasEnvironment, Requires::Requires};

/// A trait that defines the core contract for an application's runtime engine.
///
/// An `ApplicationRunTime` is the component that bridges the declarative world
/// of `ActionEffect`s with the concrete world of execution. It is responsible
/// for taking an effect, providing it with the necessary capabilities from its
/// `Environment`, and running the resulting asynchronous operation to
/// completion.
#[async_trait]
pub trait ApplicationRunTime: HasEnvironment + Send + Sync + 'static {
	/// Executes an `ActionEffect` that requires a specific capability from the
	/// environment.
	///
	/// This method is the heart of the effect execution system. It dynamically
	/// resolves the required capability (e.g., a trait object like
	/// `Arc<dyn FileSystemReader>`) from its managed environment and provides
	/// it to the effect's encapsulated function, which is then awaited.
	async fn Run<TCapability, TError, TOutput>(
		&self,
		Effect:ActionEffect<Arc<Self>, TError, TOutput>,
	) -> Result<TOutput, TError>
	where
		TCapability: ?Sized + Send + Sync,
		Self: Requires<Arc<TCapability>>,
		TError: Send + Sync + 'static,
		TOutput: Send + Sync + 'static;
}
