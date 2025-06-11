// Defines the concrete `AppRuntime` for the Mountain application, which is the
// engine responsible for executing `ActionEffect`s.

use std::sync::Arc;

use Common::{
	effect::{ActionEffect, AppRuntime as AppRuntimeTrait},
	environment::{Environment, Requires},
};
use async_trait::async_trait;
use log::{info, trace};

use crate::environment::MountainEnvironment;

/// The concrete application runtime for the `Mountain` backend.
///
/// This struct holds the application's environment and implements the
/// `AppRuntimeTrait` from the `Common` crate, providing the logic to
/// execute declarative `ActionEffect`s.
pub struct AppRuntime {
	Environment:Arc<MountainEnvironment>,
}

impl AppRuntime {
	/// Creates a new `AppRuntime` with the given `MountainEnvironment`.
	pub fn New(Environment:Arc<MountainEnvironment>) -> Self {
		info!("[AppRuntime] New instance created.");
		Self { Environment }
	}
}

#[async_trait]
impl AppRuntimeTrait for AppRuntime {
	type EnvironmentType = MountainEnvironment;

	/// Returns a shared, reference-counted pointer to the `MountainEnvironment`
	/// associated with this runtime.
	fn GetEnvironment(&self) -> Arc<Self::EnvironmentType> { Arc::clone(&self.Environment) }

	/// Executes an `ActionEffect` that requires a specific capability.
	///
	/// This method is the core of the effect execution system. It retrieves
	/// the required capability (e.g., a trait object like `Arc<dyn FsReader>`)
	/// from the environment and provides it to the effect's closure, which
	/// is then awaited.
	async fn Run<Capability, Error, Output>(
		&self,
		Effect:ActionEffect<Arc<Capability>, Error, Output>,
	) -> Result<Output, Error>
	where
		Capability: ?Sized + Send + Sync,
		Self::EnvironmentType: Requires<Arc<Capability>>,
		Error: Send + Sync + 'static,
		Output: Send + Sync + 'static, {
		trace!("[AppRuntime] Running effect...");
		let Environment = self.GetEnvironment();
		let CapabilityProvider:Arc<Capability> = Environment.Require();
		Effect.Apply(CapabilityProvider).await
	}
}
