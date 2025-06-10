use async_trait::async_trait;

/// @module ScmProvider
/// @description Defines the abstract service trait for integrating with source
/// control management systems like Git.
use super::dto::*;
use crate::{environment::Environment, error::CommonError};

/// An abstract service contract for an environment component that can manage
/// Source Control Management (SCM) providers contributed by extensions.
#[async_trait]
pub trait ScmProvider: Environment + Send + Sync {
	/// Registers a new SCM provider with the host.
	///
	/// @param ProviderData - A DTO containing metadata about the provider, such
	/// as its label and root URI. @returns A `Result` containing a unique
	/// handle (u32) for the new provider.
	async fn RegisterScmProvider(&self, ProviderData:ScmProviderDto) -> Result<u32, CommonError>;

	/// Updates the properties of an SCM resource group (e.g., "Changes").
	///
	/// @param ProviderHandle - The handle of the SCM provider that owns the
	/// group. @param GroupData - A DTO containing the updated state of the
	/// group.
	async fn UpdateScmGroup(&self, ProviderHandle:u32, GroupData:ScmGroupDto) -> Result<(), CommonError>;

	/// Updates the state of individual resources within an SCM group.
	///
	/// @param ProviderHandle - The handle of the SCM provider.
	/// @param GroupId - The ID of the group to update.
	/// @param Resources - A vector of resource DTOs to add, update, or remove.
	async fn UpdateScmGroupResources(
		&self,
		ProviderHandle:u32,
		GroupId:String,
		Resources:Vec<ScmResourceDto>,
	) -> Result<(), CommonError>;

	/// Retrieves the current value of the SCM input box.
	///
	/// @param ProviderHandle - The handle of the SCM provider.
	/// @returns The commit message entered by the user.
	async fn GetInputBoxValue(&self, ProviderHandle:u32) -> Result<String, CommonError>;
}
