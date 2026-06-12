//! # Document Service
//!
//! Defines the abstract contract for the Document service, responsible for
//! managing the lifecycle and content of text documents. Includes the
//! `DocumentProvider` trait and `ActionEffect` constructors for all document
//! operations.

// --- Trait Definition ---
/// Trait for managing document lifecycle and content.
pub mod DocumentProvider;

// --- Effect Constructors ---
/// Effect constructor for applying incremental changes to a document.
pub mod ApplyDocumentChanges;

/// Effect constructor for opening a document.
pub mod OpenDocument;

/// Effect constructor for saving all open documents.
pub mod SaveAllDocuments;

/// Effect constructor for saving a single document.
pub mod SaveDocument;

/// Effect constructor for saving a document with a new name.
pub mod SaveDocumentAs;
