use std::{path::PathBuf, sync::Arc};

use super::{UiProvider::UiProvider, dto::SaveDialogOptionsDto};
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn ShowSaveDialog<Runtime>(
	Options:Option<SaveDialogOptionsDto>,
) -> ActionEffect<Arc<Runtime>, CommonError, Option<PathBuf>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn UiProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let OptionsClone = Options.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn UiProvider> = Environment.Require();
			Provider.ShowSaveDialog(OptionsClone).await
		})
	}))
}
