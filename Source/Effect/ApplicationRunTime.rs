//! # ApplicationRunTime Trait
//!
//! Defines the core `ApplicationRunTimeTrait`, which is the contract for any
//! "engine" capable of executing `ActionEffect`s.

use std::sync::Arc;

use async_trait::async_trait;

use super::ActionEffect::ActionEffect;
use crate::{
	Environment::{HasEnvironment::HasEnvironment, Requires::Requires},
	Error::CommonError::CommonError,
};

/// The core trait for any runtime capable of executing `ActionEffect`s.
#[async_trait]
pub trait ApplicationRunTime: HasEnvironment + Send + Sync + 'static {
	/// Executes an effect using the environment provided by the runtime.
	///
	/// The runtime is responsible for acquiring the necessary capability from
	/// its environment and passing it to the effect's execution logic.
	async fn Run<TCapabilityProvider, TError, TOutput>(
		&self,

		Effect:ActionEffect<Arc<TCapabilityProvider>, TError, TOutput>,
	) -> Result<TOutput, TError>
	where
		TCapabilityProvider: ?Sized + Send + Sync + 'static,
		Self::EnvironmentType: Requires<TCapabilityProvider>,
		TError: From<CommonError> + Send + Sync + 'static,
		TOutput: Send + Sync + 'static;
}

/// A blanket implementation that allows a shared `Arc` of a runtime to also be
/// used as a runtime.
#[async_trait]
impl<TRunTime:ApplicationRunTime> ApplicationRunTime for Arc<TRunTime> {
	async fn Run<TCapabilityProvider, TError, TOutput>(
		&self,

		Effect:ActionEffect<Arc<TCapabilityProvider>, TError, TOutput>,
	) -> Result<TOutput, TError>
	where
		TCapabilityProvider: ?Sized + Send + Sync + 'static,
		Self::EnvironmentType: Requires<TCapabilityProvider>,
		TError: From<CommonError> + Send + Sync + 'static,
		TOutput: Send + Sync + 'static, {
		(**self).Run(Effect).await
	}
}
