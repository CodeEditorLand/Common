//! # SourceControlManagementProvider Trait
//!
//! Defines the abstract service trait for integrating with source control
//! management systems like Git.

use async_trait::async_trait;

use super::DTO::{
	SourceControlManagementGroupDTO::SourceControlManagementGroupDTO,
	SourceControlManagementProviderDTO::SourceControlManagementProviderDTO,
	SourceControlManagementResourceDTO::SourceControlManagementResourceDTO,
};
use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// Source Control Management (SCM) providers contributed by extensions.
#[async_trait]
pub trait SourceControlManagementProvider: Environment + Send + Sync {
	/// Registers a new SCM provider with the host.
	///
	/// # Parameters
	/// * `ProviderData`: A DTO containing metadata about the provider, such as
	///   its label and root URI.
	///
	/// # Returns
	/// A `Result` containing a unique handle (u32) for the new provider.
	async fn RegisterSourceControlManagementProvider(
		&self,
		ProviderData:SourceControlManagementProviderDTO,
	) -> Result<u32, CommonError>;

	/// Updates the properties of an SCM resource group (e.g., "Changes").
	///
	/// # Parameters
	/// * `ProviderHandle`: The handle of the SCM provider that owns the group.
	/// * `GroupData`: A DTO containing the updated state of the group.
	async fn UpdateSourceControlManagementGroup(
		&self,
		ProviderHandle:u32,
		GroupData:SourceControlManagementGroupDTO,
	) -> Result<(), CommonError>;

	/// Updates the state of individual resources within an SCM group.
	///
	/// # Parameters
	/// * `ProviderHandle`: The handle of the SCM provider.
	/// * `GroupIdentifier`: The ID of the group to update.
	/// * `Resources`: A vector of resource DTOs to add, update, or remove.
	async fn UpdateSourceControlManagementGroupResources(
		&self,
		ProviderHandle:u32,
		GroupIdentifier:String,
		Resources:Vec<SourceControlManagementResourceDTO>,
	) -> Result<(), CommonError>;

	/// Retrieves the current value of the SCM input box (commit message).
	///
	/// # Parameters
	/// * `ProviderHandle`: The handle of the SCM provider.
	async fn GetInputBoxValue(&self, ProviderHandle:u32) -> Result<String, CommonError>;
}
