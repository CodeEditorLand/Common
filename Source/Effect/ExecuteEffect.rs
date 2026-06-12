//! # ExecuteEffect Helper
//!
//! Defines the generic `ExecuteEffect` helper function, which provides a more
//! ergonomic and readable way to run `ActionEffect`s within application logic.

use std::sync::Arc;

use super::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime};
use crate::{Environment::Requires::Requires, Error::CommonError::CommonError};

/// A generic effect execution helper that takes a runtime and an effect, and
/// executes the effect using that runtime.
///
/// Provides a more concise and declarative syntax for running effects,
/// abstracting away the direct `RunTime.Run(Effect)` call and making
/// the intent of the code clearer.
///
/// # Example
///
/// ```ignore
// A function that reads a file using an effect.
/// async fn ReadMyFile(RunTime: Arc<impl ApplicationRunTime>) ->
/// Result<Vec<u8>, CommonError> {

///     let ReadEffect =
/// FileSystem::ReadFile(PathBuf::from("/path/to/file.txt"));

///     ExecuteEffect(RunTime, ReadEffect).await
/// }

/// ```
pub async fn ExecuteEffect<TRunTime, TCapabilityProvider, TError, TOutput>(
	RunTime:Arc<TRunTime>,

	Effect:ActionEffect<Arc<TCapabilityProvider>, TError, TOutput>,
) -> Result<TOutput, TError>
where
	TRunTime: ApplicationRunTime,
	TCapabilityProvider: ?Sized + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<TCapabilityProvider>,
	TError: From<CommonError> + Send + Sync + 'static,
	TOutput: Send + Sync + 'static, {
	// The RunTime::Run method expects an effect whose closure takes the capability.
	// This now matches perfectly.
	RunTime.Run(Effect).await
}
