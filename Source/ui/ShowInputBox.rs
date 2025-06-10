use std::sync::Arc;

use super::{UiProvider::UiProvider, dto::InputBoxOptionsDto};
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn ShowInputBox<Runtime>(
	Options:Option<InputBoxOptionsDto>,
) -> ActionEffect<Arc<Runtime>, CommonError, Option<String>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn UiProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let OptionsClone = Options.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn UiProvider> = Environment.Require();
			Provider.ShowInputBox(OptionsClone).await
		})
	}))
}
