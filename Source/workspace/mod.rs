

//
// @module workspace
// @description This module defines the abstract contract for the Workspace service.
// It includes the `WorkspaceProvider` and `WorkspaceEditApplier` traits, and the
// `ActionEffect` constructors for every workspace-related operation.
//

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definitions ---
mod WorkspaceEditApplier;
mod WorkspaceProvider;

pub use self::WorkspaceEditApplier::WorkspaceEditApplier;
pub use self::WorkspaceProvider::WorkspaceProvider;

// --- Effect Constructors ---
mod ApplyWorkspaceEdit;
mod FindFilesInWorkspace;
mod GetWorkspaceConfigurationPath;
mod GetWorkspaceFolderInfo;
mod GetWorkspaceFoldersInfo;
mod GetWorkspaceName;
mod IsWorkspaceTrusted;
mod OpenFile;
mod RequestWorkspaceTrust;

pub use self::ApplyWorkspaceEdit::ApplyWorkspaceEdit;
pub use self::FindFilesInWorkspace::FindFilesInWorkspace;
pub use self::GetWorkspaceConfigurationPath::GetWorkspaceConfigurationPath;
pub use self::GetWorkspaceFolderInfo::GetWorkspaceFolderInfo;
pub use self::GetWorkspaceFoldersInfo::GetWorkspaceFoldersInfo;
pub use self::GetWorkspaceName::GetWorkspaceName;
pub use self::IsWorkspaceTrusted::IsWorkspaceTrusted;
pub use self::OpenFile::OpenFile;
pub use self::RequestWorkspaceTrust::RequestWorkspaceTrust;
