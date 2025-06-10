use std::sync::Arc;

use serde_json::Value;

use super::{UiProvider::UiProvider, dto::MessageSeverity};
use crate::{
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn ShowMessage<Runtime>(
	Severity:MessageSeverity,
	Message:String,
	OptionsValue:Value,
) -> ActionEffect<Arc<Runtime>, CommonError, Option<String>>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn UiProvider>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let MessageClone = Message.clone();
		let OptionsClone = OptionsValue.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Provider:Arc<dyn UiProvider> = Environment.Require();
			Provider.ShowMessage(Severity, MessageClone, Some(OptionsClone)).await
		})
	}))
}
