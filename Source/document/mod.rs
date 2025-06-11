

//
// @module document
// @description This module defines the abstract contract for the Document service,
// which is responsible for managing the lifecycle and content of text documents.
// It includes the `DocumentProvider` trait and the `ActionEffect` constructors
// for all document operations.
//

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod DocumentProvider;
pub use self::DocumentProvider::DocumentProvider;

// --- Effect Constructors ---
mod ApplyDocumentChanges;
mod OpenDocument;
mod SaveAllDocuments;
mod SaveDocument;
mod SaveDocumentAs;

pub use self::ApplyDocumentChanges::ApplyDocumentChanges;
pub use self::OpenDocument::OpenDocument;
pub use self::SaveAllDocuments::SaveAllDocuments;
pub use self::SaveDocument::SaveDocument;
pub use self::SaveDocumentAs::SaveDocumentAs;
