

/**
 * @module fs
 * @description This module defines the abstract contract for the Filesystem service.
 * It includes the `FsReader` and `FsWriter` traits, all related DTOs, and the
 * `ActionEffect` constructors for every filesystem operation.
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definitions ---
mod FsReader;
mod FsWriter;

pub use self::FsReader::FsReader;
pub use self::FsWriter::FsWriter;

// --- Data Transfer Objects ---
pub mod dto;

// --- Effect Constructors ---
mod Copy;
mod CreateDirectory;
mod CreateFile;
mod Delete;
mod ReadDirectory;
mod ReadFile;
mod Rename;
mod StatFile;
mod WriteFileBytes;
mod WriteFileString;

pub use self::Copy::Copy;
pub use self::CreateDirectory::CreateDirectory;
pub use self::CreateFile::CreateFile;
pub use self::Delete::Delete;
pub use self::ReadDirectory::ReadDirectory;
pub use self::ReadFile::ReadFile;
pub use self::Rename::Rename;
pub use self::StatFile::StatFile;
pub use self::WriteFileBytes::WriteFileBytes;
pub use self::WriteFileString::WriteFileString;
