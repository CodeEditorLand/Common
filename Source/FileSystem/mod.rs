//! # FileSystem Service
//!
//! Defines the abstract contract for the FileSystem service, including
//! `FileSystemReader` and `FileSystemWriter` traits, related Data Transfer
//! Objects (DTOs), and `ActionEffect` constructors for every filesystem
//! operation.

// --- Trait Definitions ---
/// Trait for reading files and listing directories.
pub mod FileSystemReader;

/// Trait for writing files and creating directories.
pub mod FileSystemWriter;

/// Trait for watching file system changes.
pub mod FileWatcherProvider;

// --- Data Transfer Objects ---
/// DTOs for the FileSystem service.
pub mod DTO;

// --- Effect Constructors ---
/// Effect constructor for copying files.
pub mod Copy;

/// Effect constructor for creating directories.
pub mod CreateDirectory;

/// Effect constructor for creating files.
pub mod CreateFile;

/// Effect constructor for deleting files or directories.
pub mod Delete;

/// Effect constructor for reading directory contents.
pub mod ReadDirectory;

/// Effect constructor for reading file contents.
pub mod ReadFile;

/// Effect constructor for renaming files or directories.
pub mod Rename;

/// Effect constructor for getting file metadata (stat).
pub mod StatFile;

/// Effect constructor for writing binary data to a file.
pub mod WriteFileBytes;

/// Effect constructor for writing string data to a file.
pub mod WriteFileString;
