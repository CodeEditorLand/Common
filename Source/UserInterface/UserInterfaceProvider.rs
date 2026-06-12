//! # UserInterfaceProvider Trait
//!
//! Defines the abstract service trait for all modal user interface
//! interactions.

use std::path::PathBuf;

use async_trait::async_trait;
use serde_json::Value;

use super::DTO::{
	InputBoxOptionsDTO::InputBoxOptionsDTO,
	MessageSeverity::MessageSeverity,
	OpenDialogOptionsDTO::OpenDialogOptionsDTO,
	QuickPickItemDTO::QuickPickItemDTO,
	QuickPickOptionsDTO::QuickPickOptionsDTO,
	SaveDialogOptionsDTO::SaveDialogOptionsDTO,
};
use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can perform
/// UI interactions that require user input, such as showing dialogs, messages,
/// and quick pick menus.
/// Implemented by `MountainEnvironment` so that the methods are
/// typically handled by sending events to the `Sky` frontend and awaiting a
/// response.
#[async_trait]
pub trait UserInterfaceProvider: Environment + Send + Sync {
	/// Shows a message to the user with a given severity and optional action
	/// buttons.
	///
	/// # Returns
	/// A `Result` containing an `Option<String>` with the title of the clicked
	/// action button, or `None` if the message was dismissed.
	async fn ShowMessage(
		&self,

		Severity:MessageSeverity,

		Message:String,

		Options:Option<Value>,
	) -> Result<Option<String>, CommonError>;

	/// Shows a dialog for opening files or folders.
	///
	/// # Returns
	/// A `Result` containing an `Option<Vec<PathBuf>>` with the selected
	/// paths, or `None` if the dialog was cancelled.
	async fn ShowOpenDialog(&self, Options:Option<OpenDialogOptionsDTO>) -> Result<Option<Vec<PathBuf>>, CommonError>;

	/// Shows a dialog for saving a file.
	///
	/// # Returns
	/// A `Result` containing the selected save path as an `Option<PathBuf>`, or
	/// `None` if the dialog was cancelled.
	async fn ShowSaveDialog(&self, Options:Option<SaveDialogOptionsDTO>) -> Result<Option<PathBuf>, CommonError>;

	/// Shows a quick pick list to the user.
	///
	/// # Returns
	/// A `Result` containing an `Option<Vec<String>>` with the labels of the
	/// selected items, or `None` if the quick pick was cancelled.
	async fn ShowQuickPick(
		&self,

		Items:Vec<QuickPickItemDTO>,

		Options:Option<QuickPickOptionsDTO>,
	) -> Result<Option<Vec<String>>, CommonError>;

	/// Shows an input box to solicit a string input from the user.
	///
	/// # Returns
	/// A `Result` containing the string entered by the user as an
	/// `Option<String>`, or `None` if the input box was cancelled.
	async fn ShowInputBox(&self, Options:Option<InputBoxOptionsDTO>) -> Result<Option<String>, CommonError>;
}
