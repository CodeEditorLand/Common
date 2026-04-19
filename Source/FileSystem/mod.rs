//! # FileSystem Service
//!
//! This module defines the abstract contract for the FileSystem service. It
//! includes the `FileSystemReader` and `FileSystemWriter` traits, all related
//! Data Transfer Objects (DTOs), and the `ActionEffect` constructors for every
//! filesystem operation.

// --- Trait Definitions ---
pub mod FileSystemReader;

pub mod FileSystemWriter;

pub mod FileWatcherProvider;

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
