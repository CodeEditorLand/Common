//! # Workspace Service
//!
//! Defines the abstract contract for the Workspace service, including the
//! `WorkspaceProvider` and `WorkspaceEditApplier` traits, and `ActionEffect`
//! constructors for every workspace-related operation.

// --- Trait Definitions ---
/// Trait for applying workspace edits (text edits across multiple files).
pub mod WorkspaceEditApplier;

/// Trait for workspace lifecycle, folders, trust, and file opening.
pub mod WorkspaceProvider;

// --- Effect Constructors ---
/// Effect constructor for applying a workspace edit.
pub mod ApplyWorkspaceEdit;

/// Effect constructor for finding files in the workspace.
pub mod FindFilesInWorkspace;

/// Effect constructor for getting the workspace configuration path.
pub mod GetWorkspaceConfigurationPath;

/// Effect constructor for getting a single workspace folder's info.
pub mod GetWorkspaceFolderInfo;

/// Effect constructor for getting all workspace folders' info.
pub mod GetWorkspaceFoldersInfo;

/// Effect constructor for getting the workspace name.
pub mod GetWorkspaceName;

/// Effect constructor for checking if the workspace is trusted.
pub mod IsWorkspaceTrusted;

/// Effect constructor for opening a file in the workspace.
pub mod OpenFile;

/// Effect constructor for requesting workspace trust.
pub mod RequestWorkspaceTrust;
