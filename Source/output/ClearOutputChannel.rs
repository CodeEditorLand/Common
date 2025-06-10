use std::sync::Arc;

use super::OutputChannelManager::OutputChannelManager;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn ClearOutputChannel<Runtime>(ChannelIdentifier:String) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn OutputChannelManager>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let IdClone = ChannelIdentifier.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Manager:Arc<dyn OutputChannelManager> = Environment.Require();
			Manager.Clear(IdClone).await
		})
	}))
}
