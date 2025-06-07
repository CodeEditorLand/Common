// Land_Common/src/workspace_effects.rs
use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use serde_json::Value; // For DTOs like GlobParam or options
use url::Url;

// Assuming WorkspaceEditDto is defined elsewhere, e.g., language_feature_effects or a common DTO module.
use crate::language_feature_effects::WorkspaceEditDto;
// Ensure AppRuntime is the correct type from your runtime module.
use crate::runtime::AppRuntime;
use crate::{
	effect::ActionEffect,
	environment::{Environment, Requires},
	errors::CommonError,
}; // For URIs of workspace folders and files

/// Trait for interacting with the workspace, such as querying workspace
/// folders, configuration, trust status, and finding files.
#[async_trait]
pub trait WorkspaceProvider: Environment {
	/// Retrieves information about all currently open workspace folders.
	/// Returns a vector of tuples: (folder_uri, folder_name, folder_index).
	async fn get_workspace_folders_info(&self) -> Result<Vec<(Url, String, usize)>, CommonError>;

	/// Retrieves information for a specific workspace folder that contains or
	/// matches the given URI.
	async fn get_workspace_folder_info(&self, uri_to_match:Url) -> Result<Option<(Url, String, usize)>, CommonError>;

	/// Gets the name of the current workspace (if any).
	/// This might be derived from the workspace configuration file or the root
	/// folder name.
	async fn get_workspace_name(&self) -> Result<Option<String>, CommonError>;

	/// Gets the path to the workspace configuration file (e.g.,
	/// `.code-workspace` file).
	async fn get_workspace_configuration_path(&self) -> Result<Option<PathBuf>, CommonError>;

	/// Checks if the current workspace is trusted.
	async fn is_workspace_trusted(&self) -> Result<bool, CommonError>;

	/// Requests the user to grant or deny trust to the current workspace.
	/// `options` can be a DTO for providing context or customization to the
	/// trust dialog. Returns `true` if trust was granted (or already granted),
	/// `false` otherwise.
	async fn request_workspace_trust(&self, options:Option<Value>) -> Result<bool, CommonError>;

	/// Finds files within the workspace matching the given criteria.
	///
	/// # Argument
	/// * `include_pattern_dto`: A DTO (as `Value`) representing glob patterns
	///   to include.
	/// * `exclude_pattern_dto`: An optional DTO (as `Value`) for glob patterns
	///   to exclude.
	/// * `max_results`: An optional limit on the number of results.
	/// * `use_ignore_files`: Whether to respect ignore files (e.g.,
	///   `.gitignore`).
	/// * `follow_symlinks`: Whether to follow symbolic links.
	async fn find_files_in_workspace(
		&self,
		include_pattern_dto:Value,
		exclude_pattern_dto:Option<Value>,
		max_results:Option<usize>,
		use_ignore_files:bool,
		follow_symlinks:bool,
	) -> Result<Vec<Url>, CommonError>;

	/// Opens a file.
	/// Note: Opening a file is often a concern of `DocumentProvider` which
	/// handles document lifecycle. This method might be for specific
	/// workspace-relative opening or for editor-less file access if distinct
	/// from document management. For now, its exact role needs clarification.
	async fn open_file(&self, path:PathBuf) -> Result<(), CommonError>;
}

/// Trait for applying workspace edits.
/// A workspace edit can include changes to multiple files, file creations,
/// deletions, or renames.
#[async_trait]
pub trait WorkspaceEditApplier: Environment {
	/// Applies the given `WorkspaceEditDto` to the workspace.
	/// Returns `true` if the edit was applied successfully, `false` otherwise
	/// (e.g., if some parts of the edit failed or were rejected).
	async fn apply_workspace_edit(
		&self,
		edit_dto:WorkspaceEditDto,
		// Consider adding metadata or options if edits can be applied with specific flags
		// or if they can trigger subsequent commands.
	) -> Result<bool, CommonError>;
}

// --- WorkspaceProvider Effect Constructors ---

/// Creates an effect to get information about all workspace folders.
pub fn get_workspace_folders_info() -> ActionEffect<Arc<AppRuntime>, CommonError, Vec<(Url, String, usize)>> {
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn WorkspaceProvider + Send + Sync> = concrete_env.require();
			provider.get_workspace_folders_info().await
		})
	}))
}

/// Creates an effect to get information for a specific workspace folder.
pub fn get_workspace_folder_info(
	uri_to_match:Url,
) -> ActionEffect<Arc<AppRuntime>, CommonError, Option<(Url, String, usize)>> {
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let uri_clone = uri_to_match.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn WorkspaceProvider + Send + Sync> = concrete_env.require();
			provider.get_workspace_folder_info(uri_clone).await
		})
	}))
}

/// Creates an effect to get the name of the current workspace.
pub fn get_workspace_name() -> ActionEffect<Arc<AppRuntime>, CommonError, Option<String>> {
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn WorkspaceProvider + Send + Sync> = concrete_env.require();
			provider.get_workspace_name().await
		})
	}))
}

/// Creates an effect to get the path of the workspace configuration file.
pub fn get_workspace_configuration_path() -> ActionEffect<Arc<AppRuntime>, CommonError, Option<PathBuf>> {
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn WorkspaceProvider + Send + Sync> = concrete_env.require();
			provider.get_workspace_configuration_path().await
		})
	}))
}

/// Creates an effect to check if the current workspace is trusted.
pub fn is_workspace_trusted() -> ActionEffect<Arc<AppRuntime>, CommonError, bool> {
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn WorkspaceProvider + Send + Sync> = concrete_env.require();
			provider.is_workspace_trusted().await
		})
	}))
}

/// Creates an effect to request workspace trust.
pub fn request_workspace_trust(options_dto:Option<Value>) -> ActionEffect<Arc<AppRuntime>, CommonError, bool> {
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let opts_clone = options_dto.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn WorkspaceProvider + Send + Sync> = concrete_env.require();
			provider.request_workspace_trust(opts_clone).await
		})
	}))
}

/// Creates an effect to find files within the workspace.
pub fn find_files_in_workspace(
	include_pattern_dto:Value,
	exclude_pattern_dto:Option<Value>,
	max_results:Option<usize>,
	use_ignore_files:bool,
	follow_symlinks:bool,
) -> ActionEffect<Arc<AppRuntime>, CommonError, Vec<Url>> {
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let include_clone = include_pattern_dto.clone();
		let exclude_clone = exclude_pattern_dto.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn WorkspaceProvider + Send + Sync> = concrete_env.require();
			provider
				.find_files_in_workspace(
					include_clone,
					exclude_clone,
					max_results,      // Option<usize> is Copy
					use_ignore_files, // bool is Copy
					follow_symlinks,  // bool is Copy
				)
				.await
		})
	}))
}

/// Creates an effect to open a file (potentially workspace-relative).
pub fn open_workspace_file(path:PathBuf) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let p_clone = path.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let provider:Arc<dyn WorkspaceProvider + Send + Sync> = concrete_env.require();
			provider.open_file(p_clone).await
		})
	}))
}

// --- WorkspaceEditApplier Effect Constructor ---

/// Creates an effect to apply a workspace edit.
pub fn apply_workspace_edit_effect(edit_dto:WorkspaceEditDto) -> ActionEffect<Arc<AppRuntime>, CommonError, bool> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let edit_dto_clone = edit_dto.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			// Ensure the concrete environment (e.g., MountainEnvironment) implements
			// Requires<Arc<dyn WorkspaceEditApplier>>
			let applier:Arc<dyn WorkspaceEditApplier + Send + Sync> = concrete_env.require();
			applier.apply_workspace_edit(edit_dto_clone).await
		})
	}))
}
