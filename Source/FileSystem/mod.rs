//! # FileSystem Service
//!
//! This module defines the abstract contract for the FileSystem service. It
//! includes the `FileSystemReader` and `FileSystemWriter` traits, all related
//! Data Transfer Objects (DTOs), and the `ActionEffect` constructors for every
//! filesystem operation.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definitions ---
pub mod FileSystemReader;
pub mod FileSystemWriter;

// pub use self::{FileSystemReader::FileSystemReader, FileSystemWriter::FileSystemWriter};

// --- Data Transfer Objects ---
pub mod DTO;

// --- Effect Constructors ---
pub mod Copy;
pub mod CreateDirectory;
pub mod CreateFile;
pub mod Delete;
pub mod ReadDirectory;
pub mod ReadFile;
pub mod Rename;
pub mod StatFile;
pub mod WriteFileBytes;
pub mod WriteFileString;

// pub use self::{
// 	Copy::Copy,
// 	CreateDirectory::CreateDirectory,
// 	CreateFile::CreateFile,
// 	Delete::Delete,
// 	ReadDirectory::ReadDirectory,
// 	ReadFile::ReadFile,
// 	Rename::Rename,
// 	StatFile::StatFile,
// 	WriteFileBytes::WriteFileBytes,
// 	WriteFileString::WriteFileString,
// };
