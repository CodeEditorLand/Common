//! # StatusBarProvider Trait
//!
//! Defines the abstract service trait for creating and managing status bar
//! items contributed by extensions.

use async_trait::async_trait;
use serde_json::Value;

use super::DTO::StatusBarEntryDTO::StatusBarEntryDTO;
use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// the state and rendering of status bar entries.
///
/// This trait is implemented by `MountainEnvironment` and defines the
/// operations that `Cocoon` can request from the host to manage the UI state of
/// the status bar.
#[async_trait]
pub trait StatusBarProvider: Environment + Send + Sync {
	/// Informs the host to create a new status bar entry or update an existing
	/// one. The `EntryIdentifier` within the DTO is used to identify the
	/// entry.
	///
	/// # Parameters
	/// * `Entry`: The DTO containing the complete state of the status bar item.
	async fn SetEntry(&self, Entry:StatusBarEntryDTO) -> Result<(), CommonError>;

	/// Informs the host to dispose of (remove) a status bar entry from the UI.
	///
	/// # Parameters
	/// * `EntryIdentifier`: The unique identifier of the entry to remove.
	async fn DisposeEntry(&self, EntryIdentifier:String) -> Result<(), CommonError>;

	/// This method is called *by* the host *to* the extension host (`Cocoon`)
	/// when a dynamic tooltip needs to be resolved for a status bar item.
	///
	/// # Parameters
	/// * `EntryIdentifier`: The unique identifier of the entry for which to
	///   provide a tooltip.
	///
	/// # Returns
	/// A `Result` containing an optional DTO for the tooltip (e.g.,
	/// `IMarkdownStringDTO`).
	async fn ProvideTooltip(&self, EntryIdentifier:String) -> Result<Option<Value>, CommonError>;
}
