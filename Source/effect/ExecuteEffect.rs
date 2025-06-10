//! Defines the generic `ExecuteEffect` helper function, providing a more
//! ergonomic way to run `ActionEffect`s.

use std::sync::Arc;

use super::{ActionEffect, AppRuntime};
use crate::environment::Requires;

/// A generic effect execution helper that takes a runtime and an effect, and
/// executes the effect using the runtime.
///
/// This function provides a more concise way to run effects, abstracting away
/// the direct call to `Runtime.Run(Effect)`.
///
/// # Example
/// ```ignore
// async fn MyFileReadingFunction(Runtime: Arc<impl AppRuntime>) -> Result<Vec<u8>, CommonError> {
//     let Effect = Fs::ReadFile(PathBuf::from("/my/file.txt"));
//     ExecuteEffect(Runtime, Effect).await
// }
/// ```
pub async fn ExecuteEffect<Runtime, Capability, Error, Output>(
	Runtime:Arc<Runtime>,
	Effect:ActionEffect<Arc<Capability>, Error, Output>,
) -> Result<Output, Error>
where
	Runtime: AppRuntime,
	Capability: ?Sized + Send + Sync,
	Runtime::EnvironmentType: Requires<Arc<Capability>>,
	Error: Send + Sync + 'static,
	Output: Send + Sync + 'static, {
	Runtime.Run(Effect).await
}
