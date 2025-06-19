//! # ApplyWorkSpaceEdit Effect
//!
//! Defines the `ActionEffect` for applying a complex, multi-file workspace
//! edit.

use std::sync::Arc;

use super::WorkSpaceEditApplier::WorkSpaceEditApplier;
use crate::{
	DTO::WorkSpaceEditDTO::WorkSpaceEditDTO,
	Effect::ActionEffect::ActionEffect,
	Error::CommonError::CommonError,
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
pub fn ApplyWorkSpaceEdit(EditDTO:WorkSpaceEditDTO) -> ActionEffect<Arc<dyn WorkSpaceEditApplier>, CommonError, bool> {
	ActionEffect::New(Arc::new(move |Applier:Arc<dyn WorkSpaceEditApplier>| {
		let EditDTOClone = EditDTO.clone();
		Box::pin(async move { Applier.ApplyWorkSpaceEdit(EditDTOClone).await })
	}))
}
