//! # Workspace Service
//!
//! This module defines the abstract contract for the Workspace service. It
//! includes the `WorkspaceProvider` and `WorkspaceEditApplier` traits, and
//! the `ActionEffect` constructors for every workspace-related operation.

// --- Trait Definitions ---
pub mod WorkspaceEditApplier;

pub mod WorkspaceProvider;

// --- Effect Constructors ---
pub mod ApplyWorkspaceEdit;

pub mod FindFilesInWorkspace;

pub mod GetWorkspaceConfigurationPath;

pub mod GetWorkspaceFolderInfo;

pub mod GetWorkspaceFoldersInfo;

pub mod GetWorkspaceName;

pub mod IsWorkspaceTrusted;

pub mod OpenFile;

pub mod RequestWorkspaceTrust;
