use async_trait::async_trait;

/// @module WorkspaceEditApplier
/// @description Defines the `WorkspaceEditApplier` trait for applying batch
/// edits across the workspace.
use crate::dto::WorkspaceEditDto;
use crate::{environment::Environment, error::CommonError};

/// An abstract service contract for an environment component that can apply a
/// `WorkspaceEdit`.
///
/// A `WorkspaceEdit` is a complex, potentially transactional operation that can
/// include text edits to multiple files, as well as file system operations like
/// creating, deleting, or renaming files.
#[async_trait]
pub trait WorkspaceEditApplier: Environment + Send + Sync {
	/// Applies the given `WorkspaceEditDto` to the workspace.
	///
	/// @param EditDto - The DTO representing the batch of edits to apply.
	/// @returns A `Result` indicating whether the entire edit was applied
	/// successfully.   A `false` value may indicate a partial success or a
	/// user cancellation.
	async fn ApplyWorkspaceEdit(&self, EditDto:WorkspaceEditDto) -> Result<bool, CommonError>;
}
