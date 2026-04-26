//! # FileSystemStatDTO
//!
//! Defines the Data Transfer Object for file and directory metadata.

use serde::{Deserialize, Serialize};

/// A serializable struct that represents metadata for a filesystem entry.
///
/// This DTO is returned by the `StatFile` effect and is analogous to VS Code's
/// `FileStat` interface, providing essential information like file type, size,

/// and modification times.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileSystemStatDTO {
	/// A bitmask representing the type of the file.
	/// @see FileTypeDTO
	pub FileType:u8,

	/// The creation time of the file in milliseconds since the UNIX epoch.
	pub CreationTime:u64,

	/// The last modification time of the file in milliseconds since the UNIX
	/// epoch.
	pub ModificationTime:u64,

	/// The size of the file in bytes.
	pub Size:u64,

	/// An optional bitmask representing the file's permissions. This is
	/// platform-specific and may not always be available.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Permissions:Option<u32>,
}
