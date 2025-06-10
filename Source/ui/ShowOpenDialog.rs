use std::{path::PathBuf, sync::Arc};

use super::{UiProvider::UiProvider, dto::OpenDialogOptionsDto};
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn ShowOpenDialog<Runtime>(
	Options:Option<OpenDialogOptionsDto>,
) -> ActionEffect<Arc<Runtime>, CommonError, Option<Vec<PathBuf>>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn UiProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let OptionsClone = Options.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn UiProvider> = Environment.Require();
			Provider.ShowOpenDialog(OptionsClone).await
		})
	}))
}
