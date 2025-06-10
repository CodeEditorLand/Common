use std::sync::Arc;

use super::OutputChannelManager::OutputChannelManager;
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn RegisterOutputChannel<Runtime>(
	Name:String,
	LanguageIdentifier:Option<String>,
) -> ActionEffect<Arc<Runtime>, CommonError, String>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn OutputChannelManager>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let NameClone = Name.clone();
		let LangIdClone = LanguageIdentifier.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Manager:Arc<dyn OutputChannelManager> = Environment.Require();
			Manager.RegisterChannel(NameClone, LangIdClone).await
		})
	}))
}
