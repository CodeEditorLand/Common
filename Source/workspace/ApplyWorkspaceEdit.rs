use std::sync::Arc;

use super::WorkspaceEditApplier::WorkspaceEditApplier;
use crate::{
	dto::WorkspaceEditDto,
	effect::{ActionEffect, AppRuntime},
	environment::Requires,
	error::CommonError,
};

pub fn ApplyWorkspaceEdit<Runtime>(EditDto:WorkspaceEditDto) -> ActionEffect<Arc<Runtime>, CommonError, bool>
where
	Runtime: AppRuntime + Send + Sync + 'static,
	Runtime::EnvironmentType: Requires<Arc<dyn WorkspaceEditApplier>>, {
	ActionEffect::New(Arc::new(move |Runtime:Arc<Runtime>| {
		let EditDtoClone = EditDto.clone();
		Box::pin(async move {
			let Environment = Runtime.GetEnvironment();
			let Applier:Arc<dyn WorkspaceEditApplier> = Environment.Require();
			Applier.ApplyWorkspaceEdit(EditDtoClone).await
		})
	}))
}
