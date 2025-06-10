use std::sync::Arc;

use super::OutputChannelManager::OutputChannelManager;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn ReplaceOutputChannelContent<Runtime>(
	ChannelIdentifier:String,
	Value:String,
) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn OutputChannelManager>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let IdClone = ChannelIdentifier.clone();
		let ValueClone = Value.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Manager:Arc<dyn OutputChannelManager> = Environment.Require();
			Manager.Replace(IdClone, ValueClone).await
		})
	}))
}
