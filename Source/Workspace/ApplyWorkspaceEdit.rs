//! # ApplyWorkspaceEdit Effect
//!
//! Defines the `ActionEffect` for applying a complex, multi-file workspace
//! edit.

use std::sync::Arc;

use super::WorkspaceEditApplier::WorkspaceEditApplier;
use crate::{
	DTO::WorkspaceEditDTO::WorkspaceEditDTO,
	Effect::ActionEffect::ActionEffect,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will apply a `WorkspaceEdit` to the
/// workspace.
/// A `WorkspaceEdit` is a batch of operations that can include text edits to
/// multiple files and filesystem operations like creating, deleting, or
/// renaming files. This effect uses the dedicated `WorkspaceEditApplier`
/// capability.
///
/// # Parameters
/// * `EditDTO`: The `WorkspaceEditDTO` representing the batch of edits to
///   apply.
///
/// # Returns
/// An `ActionEffect` that resolves with a `bool` indicating whether the entire
/// edit was applied successfully.
pub fn ApplyWorkspaceEdit(EditDTO:WorkspaceEditDTO) -> ActionEffect<Arc<dyn WorkspaceEditApplier>, CommonError, bool> {
	ActionEffect::New(Arc::new(move |Applier:Arc<dyn WorkspaceEditApplier>| {
		let EditDTOClone = EditDTO.clone();

		Box::pin(async move { Applier.ApplyWorkspaceEdit(EditDTOClone).await })
	}))
}
