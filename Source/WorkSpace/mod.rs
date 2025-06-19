//! # WorkSpace Service
//!
//! This module defines the abstract contract for the WorkSpace service. It
//! includes the `WorkSpaceProvider` and `WorkSpaceEditApplier` traits, and
//! the `ActionEffect` constructors for every workspace-related operation.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definitions ---
pub mod WorkSpaceEditApplier;
pub mod WorkSpaceProvider;

// pub use self::{WorkSpaceEditApplier::WorkSpaceEditApplier,
// WorkSpaceProvider::WorkSpaceProvider};

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

// pub use self::{
// 	ApplyWorkSpaceEdit::ApplyWorkSpaceEdit,
// 	FindFilesInWorkSpace::FindFilesInWorkSpace,
// 	GetWorkSpaceConfigurationPath::GetWorkSpaceConfigurationPath,
// 	GetWorkSpaceFolderInfo::GetWorkSpaceFolderInfo,
// 	GetWorkSpaceFoldersInfo::GetWorkSpaceFoldersInfo,
// 	GetWorkSpaceName::GetWorkSpaceName,
// 	IsWorkSpaceTrusted::IsWorkSpaceTrusted,
// 	OpenFile::OpenFile,
// 	RequestWorkSpaceTrust::RequestWorkSpaceTrust,
// };
