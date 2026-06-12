// File: Common/Source/SourceControlManagement/SourceControlManagementProvider.
// rs Role: Defines the abstract service trait for Source Control Management
// (SourceControlManagement). Responsibilities:
//   - Provide a contract for creating and managing SourceControlManagement
//     providers.
//   - Provide a contract for updating SourceControlManagement groups and their
//     resources.
//   - Provide a contract for managing the SourceControlManagement input box.

//! # SourceControlManagementProvider Trait
//!
//! Defines the abstract service trait for integrating with source control
//! management systems like Git.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// Source Control Management (SourceControlManagement) providers contributed by
/// extensions.
#[async_trait]
pub trait SourceControlManagementProvider: Environment + Send + Sync {
	/// Creates a new SourceControlManagement provider in the host.
	///
	/// # Parameters
	/// * `ProviderData`: A DTO containing metadata about the provider, such as
	///   its ID, label, and root URI.
	///
	/// # Returns
	/// A `Result` containing a unique handle (`u32`) for the new provider.
	async fn CreateSourceControl(
		&self,

		// DTO: SourceControlCreateDTO
		ProviderData:Value,
	) -> Result<u32, CommonError>;

	/// Disposes of an SourceControlManagement provider, removing it and its
	/// groups from the UI.
	///
	/// # Parameters
	/// * `ProviderHandle`: The handle of the SourceControlManagement provider
	///   to dispose.
	async fn DisposeSourceControl(&self, ProviderHandle:u32) -> Result<(), CommonError>;

	/// Updates the core properties of an SourceControlManagement provider.
	/// This is used to update properties like the commit message template, the
	/// count badge, and the accept command.
	///
	/// # Parameters
	/// * `ProviderHandle`: The handle of the provider to update.
	/// * `UpdateData`: A DTO containing the properties to update.
	async fn UpdateSourceControl(
		&self,

		ProviderHandle:u32,

		// DTO: SourceControlUpdateDTO
		UpdateData:Value,
	) -> Result<(), CommonError>;

	/// Updates the properties of an SourceControlManagement resource group
	/// (e.g., "Changes").
	/// This can update the group's label, hide state, and its list of
	/// resources.
	///
	/// # Parameters
	/// * `ProviderHandle`: The handle of the SourceControlManagement provider
	///   that owns the group.
	/// * `GroupData`: A DTO containing the updated state of the group.
	async fn UpdateSourceControlGroup(
		&self,

		ProviderHandle:u32,

		// DTO: SourceControlGroupUpdateDTO
		GroupData:Value,
	) -> Result<(), CommonError>;

	/// Registers or updates the SourceControlManagement input box for a
	/// provider.
	///
	/// # Parameters
	/// * `ProviderHandle`: The handle of the SourceControlManagement provider.
	/// * `InputBoxData`: A DTO containing the state of the input box (e.g.,

	///   value, placeholder).
	async fn RegisterInputBox(
		&self,

		ProviderHandle:u32,

		// DTO: SourceControlInputBoxDTO
		InputBoxData:Value,
	) -> Result<(), CommonError>;
}
