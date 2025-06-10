use std::sync::Arc;

use super::{
	UiProvider::UiProvider,
	dto::{QuickPickItemDto, QuickPickOptionsDto},
};
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn ShowQuickPick<Runtime>(
	Items:Vec<QuickPickItemDto>,
	Options:Option<QuickPickOptionsDto>,
) -> ActionEffect<Arc<Runtime>, CommonError, Option<Vec<String>>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn UiProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let ItemsClone = Items.clone();
		let OptionsClone = Options.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn UiProvider> = Environment.Require();
			Provider.ShowQuickPick(ItemsClone, OptionsClone).await
		})
	}))
}
