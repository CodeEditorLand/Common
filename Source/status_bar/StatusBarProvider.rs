use async_trait::async_trait;
use serde_json::Value;

/// @module StatusBarProvider
/// @description Defines the abstract service trait for creating and managing
/// status bar items contributed by extensions.
use super::dto::*;
use crate::{environment::Environment, error::CommonError};

/// An abstract service contract for an environment component that can manage
/// the state and rendering of status bar entries.
///
/// This trait is implemented by `MountainEnvironment` and defines the
/// operations that `Cocoon` can request from the host to manage the UI state of
/// the status bar.
#[async_trait]
pub trait StatusBarProvider: Environment + Send + Sync {
	/// Informs the host to create a new status bar entry or update an existing
	/// one. The `EntryId` within the DTO is used to identify the entry.
	///
	/// @param Entry - The DTO containing the complete state of the status bar
	/// item.
	async fn SetEntry(&self, Entry:StatusBarEntryDto) -> Result<(), CommonError>;

	/// Informs the host to dispose of (remove) a status bar entry from the UI.
	///
	/// @param EntryId - The unique identifier of the entry to remove.
	async fn DisposeEntry(&self, EntryId:String) -> Result<(), CommonError>;

	/// This method is called *by* the host *to* the extension host (Cocoon)
	/// when a dynamic tooltip needs to be resolved for a status bar item.
	///
	/// @param EntryId - The unique identifier of the entry for which to provide
	/// a tooltip. @returns A `Result` containing an optional DTO for the
	/// tooltip   (e.g., `IMarkdownStringDto`).
	async fn ProvideTooltip(&self, EntryId:String) -> Result<Option<Value>, CommonError>;
}
