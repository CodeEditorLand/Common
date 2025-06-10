use std::path::PathBuf;

use async_trait::async_trait;
use serde_json::Value;

/// @module UiProvider
/// @description Defines the abstract service trait for all user interface
/// interactions.
use super::dto::{
	InputBoxOptionsDto,
	MessageOptionsDto,
	MessageSeverity,
	OpenDialogOptionsDto,
	QuickPickItemDto,
	QuickPickOptionsDto,
	SaveDialogOptionsDto,
};
use crate::{environment::Environment, error::CommonError};

/// An abstract service contract for an environment component that can perform
/// UI interactions that require user input, such as showing dialogs and
/// messages.
///
/// This trait is implemented by `MountainEnvironment` and the methods are
/// typically handled by sending events to the `Wind/Sky` frontend and awaiting
/// a response.
#[async_trait]
pub trait UiProvider: Environment + Send + Sync {
	/// Shows a message to the user with a given severity and optional buttons.
	/// @returns A `Result` containing the title of the clicked button, or
	/// `None` if dismissed.
	async fn ShowMessage(
		&self,
		Severity:MessageSeverity,
		Message:String,
		Options:Option<Value>,
	) -> Result<Option<String>, CommonError>;

	/// Shows a dialog for opening files or folders.
	/// @returns A `Result` containing a vector of selected paths, or `None` if
	/// cancelled.
	async fn ShowOpenDialog(&self, Options:Option<OpenDialogOptionsDto>) -> Result<Option<Vec<PathBuf>>, CommonError>;

	/// Shows a dialog for saving a file.
	/// @returns A `Result` containing the selected save path, or `None` if
	/// cancelled.
	async fn ShowSaveDialog(&self, Options:Option<SaveDialogOptionsDto>) -> Result<Option<PathBuf>, CommonError>;

	/// Shows a quick pick list to the user.
	/// @returns A `Result` containing a vector of the labels of the selected
	/// items, or `None` if cancelled.
	async fn ShowQuickPick(
		&self,
		Items:Vec<QuickPickItemDto>,
		Options:Option<QuickPickOptionsDto>,
	) -> Result<Option<Vec<String>>, CommonError>;

	/// Shows an input box to the user to solicit a string input.
	/// @returns A `Result` containing the string entered by the user, or `None`
	/// if cancelled.
	async fn ShowInputBox(&self, Options:Option<InputBoxOptionsDto>) -> Result<Option<String>, CommonError>;
}
