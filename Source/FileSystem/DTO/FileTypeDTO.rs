//! # FileTypeDTO
//!
//! Defines the Data Transfer Object enum for representing the type of a
//! filesystem entry.

use serde::{Deserialize, Serialize};

/// The type of a filesystem entry.
/// This is a C-like enum with an explicit `u8` representation. The values are
/// chosen to align directly with VS Code's internal `FileType` enum, ensuring
/// seamless interoperability across the IPC boundary. It is used as a bitmask
/// to allow for combinations (e.g., a symbolic link to a directory).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FileTypeDTO {
	/// The file type is unknown.
	Unknown = 0,

	/// A regular file.
	File = 1,

	/// A directory.
	Directory = 2,

	/// A symbolic link.
	SymbolicLink = 64,
}
