//! # WorkSpaceEditApplier Trait
//!
//! Defines the `WorkSpaceEditApplier` trait for applying batch edits across
//! the workspace.

use async_trait::async_trait;

// Note: WorkSpaceEditDTO is defined in `language_feature::DTO` as it's
// most commonly used there, but it is a general-purpose DTO.
use crate::LanguageFeature::DTO::WorkSpaceEditDTO;
use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can apply a
/// `WorkSpaceEdit`.
///
/// A `WorkSpaceEdit` is a complex, potentially transactional operation that can
/// include text edits to multiple files, as well as file system operations like
/// creating, deleting, or renaming files. This trait isolates the complex
/// logic of applying such edits.
#[async_trait]
pub trait WorkSpaceEditApplier: Environment + Send + Sync {
	/// Applies the given `WorkSpaceEditDTO` to the workspace.
	///
	/// # Parameters
	/// * `EditDTO`: The DTO representing the batch of edits to apply.
	///
	/// # Returns
	/// A `Result` indicating whether the entire edit was applied
	/// successfully. A `false` value may indicate a partial success or a user
	/// cancellation of one of the steps.
	async fn ApplyWorkSpaceEdit(&self, EditDTO:WorkSpaceEditDTO) -> Result<bool, CommonError>;
}
