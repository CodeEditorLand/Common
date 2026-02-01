//! # Document Service
//!
//! This module defines the abstract contract for the Document service, which is
//! responsible for managing the lifecycle and content of text documents. It
//! includes the `DocumentProvider` trait and the `ActionEffect` constructors
//! for all document operations.

// --- Trait Definition ---
pub mod DocumentProvider;

// --- Effect Constructors ---
pub mod ApplyDocumentChanges;

pub mod OpenDocument;

pub mod SaveAllDocuments;

pub mod SaveDocument;

pub mod SaveDocumentAs;
