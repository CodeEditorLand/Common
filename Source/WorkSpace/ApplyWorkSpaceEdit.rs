//! # ApplyWorkSpaceEdit Effect
//!
//! Defines the `ActionEffect` for applying a complex, multi-file workspace
//! edit.

use std::sync::Arc;

use super::WorkSpaceEditApplier::WorkSpaceEditApplier;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
	LanguageFeature::DTO::WorkSpaceEditDTO,
};

/// Creates an effect that, when executed, will apply a `WorkSpaceEdit` to the
/// workspace.
///
/// A `WorkSpaceEdit` is a batch of operations that can include text edits to
/// multiple files and filesystem operations like creating, deleting, or
/// renaming files. This effect uses the dedicated `WorkSpaceEditApplier`
/// capability.
///
/// # Parameters
/// * `EditDTO`: The `WorkSpaceEditDTO` representing the batch of edits to
///   apply.
///
/// # Returns
/// An `ActionEffect` that resolves with a `bool` indicating whether the entire
/// edit was applied successfully.
pub fn ApplyWorkSpaceEdit<TRunTime>(EditDTO:WorkSpaceEditDTO) -> ActionEffect<Arc<TRunTime>, CommonError, bool>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn WorkSpaceEditApplier>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let EditDTOClone = EditDTO.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Applier:Arc<dyn WorkSpaceEditApplier> = Environment.Require();
			Applier.ApplyWorkSpaceEdit(EditDTOClone).await
		})
	}))
}
