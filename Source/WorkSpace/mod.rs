//! # WorkSpace Service
//!
//! This module defines the abstract contract for the WorkSpace service. It
//! includes the `WorkSpaceProvider` and `WorkSpaceEditApplier` traits, and
//! the `ActionEffect` constructors for every workspace-related operation.

// --- Trait Definitions ---
pub mod WorkSpaceEditApplier;

pub mod WorkSpaceProvider;

// --- Effect Constructors ---
pub mod ApplyWorkSpaceEdit;

pub mod FindFilesInWorkSpace;

pub mod GetWorkSpaceConfigurationPath;

pub mod GetWorkSpaceFolderInfo;

pub mod GetWorkSpaceFoldersInfo;

pub mod GetWorkSpaceName;

pub mod IsWorkSpaceTrusted;

pub mod OpenFile;

pub mod RequestWorkSpaceTrust;
