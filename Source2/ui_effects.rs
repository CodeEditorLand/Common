// Land_Common/src/ui_effects.rs
use std::path::PathBuf; // For file dialog results
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value; // For flexible options DTOs if passed as raw Value

// Ensure AppRuntime is the correct type from your runtime module.
use crate::runtime::AppRuntime;
use crate::{
	effect::ActionEffect,
	environment::{Environment, Requires},
	errors::CommonError,
};
// use url::Url; // Only if open_external_url is added

// --- UI Interaction DTOs ---

/// Severity level for messages shown to the user.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageSeverity {
	Info,
	Warning,
	Error,
}

/// Options for displaying a message to the user.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MessageOptions {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title:Option<String>, // Optional custom title for the message dialog
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modal:Option<bool>, // If true, the message dialog blocks other UI interactions.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail:Option<String>, // Additional detail text for the message.
	/// A list of action buttons (titles) to display.
	/// If provided, `UiProvider::show_message` is expected to return the title
	/// of the selected item.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub items:Option<Vec<String>>,
	// VS Code also has `actions: MessageItem[]` which are more structured than just strings.
	// For simplicity, `items: Vec<String>` is used here.
}

/// Defines a filter for file dialogs.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileFilter {
	pub name:String,            // User-friendly name for the filter (e.g., "Image Files").
	pub extensions:Vec<String>, // Associated file extensions (e.g., `["jpg", "png"]`).
}

/// Base options common to both open and save file dialogs.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DialogOptions {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title:Option<String>, // Custom title for the dialog window.
	/// Default path or filename for the dialog.
	/// For Tauri dialogs, this is typically a string path.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default_path:Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filters:Option<Vec<FileFilter>>, // File filters to apply.
}

/// Options for the "Open File/Folder" dialog.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenDialogOptions {
	#[serde(flatten)]
	pub base:DialogOptions, // Common dialog options.
	/// Allow selection of multiple files/folders. Corresponds to
	/// `canSelectMany`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub multiple:Option<bool>,
	/// Allow selection of folders instead of files. Corresponds to
	/// `canSelectFolders`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub directory:Option<bool>,
	/// (Tauri-specific) Whether to allow recursive directory selection if
	/// `directory` is true.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub recursive:Option<bool>,
}

/// Options for the "Save File" dialog.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SaveDialogOptions {
	#[serde(flatten)]
	pub base:DialogOptions, /* Common dialog options.
	                         * Additional save-specific options can be added here if needed.
	                         * e.g., default filename, confirm overwrite prompts. */
}

/// Represents a single item in a quick pick list.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuickPickItem {
	pub label:String, // The primary text displayed for the item.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description:Option<String>, // Secondary text, often shown to the right of the label.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail:Option<String>, // Tertiary text, often shown below the label and description.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub picked:Option<bool>, // If true, this item is pre-selected.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub always_show:Option<bool>, /* If true, this item is always shown even if it doesn't match filter text.
	                   * VS Code's QuickPickItem can also have a `kind` (e.g., separator) and `buttons`. */
}

/// Options for configuring a quick pick UI.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuickPickOptions {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title:Option<String>, // Optional title for the quick pick UI.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub place_holder:Option<String>, // Placeholder text for the input field.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub can_pick_many:Option<bool>, // If true, multiple items can be selected.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ignore_focus_out:Option<bool>, /* If true, the quick pick UI does not close when it loses focus.
	                                    * Other options from VS Code: matchOnDescription, matchOnDetail,
	                                    * onDidSelectItem. */
}

/// Options for configuring an input box UI.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct InputBoxOptions {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title:Option<String>, // Optional title for the input box.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub place_holder:Option<String>, // Placeholder text for the input field.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value:Option<String>, // Pre-filled value for the input field.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt:Option<String>, // Descriptive text shown below the input field.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub password:Option<bool>, // If true, the input is treated as a password (e.g., masked).
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ignore_focus_out:Option<bool>, /* If true, the input box does not close when it loses focus.
	                                    * `validateInput` is a callback in VS Code's API and cannot be directly
	                                    * serialized. If validation is needed,
	                                    * it would typically be handled by the frontend (Sky)
	                                    * or via a separate validation step/command if server-side validation is
	                                    * required. */
}

/// Trait for an environment component that provides UI interaction
/// capabilities. These methods typically trigger UI elements on the frontend
/// (Sky) and may involve asynchronous request-response patterns for user input.
#[async_trait]
pub trait UiProvider: Environment {
	/// Shows a message to the user.
	///
	/// # Arguments
	/// * `severity`: The severity of the message (Info, Warning, Error).
	/// * `message`: The main text of the message.
	/// * `options`: Optional `MessageOptions` DTO (passed as `Value` for
	///   flexibility from RPC) to customize the message dialog (e.g., modality,
	///   detail, action buttons).
	///
	/// # Returns
	/// `Ok(Some(String))` containing the title of the selected action item if
	/// `options.items` were provided and one was chosen. `Ok(None)` if no
	/// items were provided, or the dialog was dismissed without selecting an
	/// item.
	async fn show_message(
		&self,
		severity:MessageSeverity,
		message:String,
		options:Option<Value>,
	) -> Result<Option<String>, CommonError>;

	/// Shows a dialog for opening files or folders.
	///
	/// # Returns
	/// `Ok(Some(Vec<PathBuf>))` containing the selected paths if the user
	/// confirms. `Ok(None)` if the dialog is cancelled.
	async fn show_open_dialog(&self, options:Option<OpenDialogOptions>) -> Result<Option<Vec<PathBuf>>, CommonError>;

	/// Shows a dialog for saving a file.
	///
	/// # Returns
	/// `Ok(Some(PathBuf))` containing the selected path if the user confirms.
	/// `Ok(None)` if the dialog is cancelled.
	async fn show_save_dialog(&self, options:Option<SaveDialogOptions>) -> Result<Option<PathBuf>, CommonError>;

	/// Shows a quick pick list to the user, allowing selection of one or more
	/// items.
	///
	/// # Returns
	/// `Ok(Some(Vec<String>))` containing the labels of the selected items if
	/// the user confirms. For single-pick, the `Vec` will contain one item.
	/// For multi-pick, it can contain multiple. `Ok(None)` if the quick pick
	/// is cancelled.
	async fn show_quick_pick(
		&self,
		items:Vec<QuickPickItem>,
		options:Option<QuickPickOptions>,
	) -> Result<Option<Vec<String>>, CommonError>;

	/// Shows an input box to the user to solicit a string input.
	///
	/// # Returns
	/// `Ok(Some(String))` containing the user's input if confirmed.
	/// `Ok(None)` if the input box is cancelled or no input is provided.
	async fn show_input_box(&self, options:Option<InputBoxOptions>) -> Result<Option<String>, CommonError>;

	// TODO: Consider adding `open_external_url(&self, url: Url) -> Result<bool,
	// CommonError>;`
}

// --- Effect Constructors ---

/// Creates an effect to show a message to the user.
pub fn show_message(
	severity:MessageSeverity,
	message:String,
	options_value:Value, // Options DTO serialized as JSON Value
) -> ActionEffect<Arc<AppRuntime>, CommonError, Option<String>> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let message_clone = message.clone();
		let options_clone = options_value.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let ui_provider:Arc<dyn UiProvider + Send + Sync> = concrete_env.require();
			// Pass Some(options_clone) to match trait signature if it takes Option<Value>
			ui_provider.show_message(severity, message_clone, Some(options_clone)).await
		})
	}))
}

/// Creates an effect to show an "Open File/Folder" dialog.
pub fn show_open_dialog(
	options:Option<OpenDialogOptions>,
) -> ActionEffect<Arc<AppRuntime>, CommonError, Option<Vec<PathBuf>>> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let options_clone = options.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let ui_provider:Arc<dyn UiProvider + Send + Sync> = concrete_env.require();
			ui_provider.show_open_dialog(options_clone).await
		})
	}))
}

/// Creates an effect to show a "Save File" dialog.
pub fn show_save_dialog(
	options:Option<SaveDialogOptions>,
) -> ActionEffect<Arc<AppRuntime>, CommonError, Option<PathBuf>> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let options_clone = options.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let ui_provider:Arc<dyn UiProvider + Send + Sync> = concrete_env.require();
			ui_provider.show_save_dialog(options_clone).await
		})
	}))
}

/// Creates an effect to show a quick pick list.
pub fn show_quick_pick(
	items:Vec<QuickPickItem>,
	options:Option<QuickPickOptions>,
) -> ActionEffect<Arc<AppRuntime>, CommonError, Option<Vec<String>>> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let items_clone = items.clone();
		let options_clone = options.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let ui_provider:Arc<dyn UiProvider + Send + Sync> = concrete_env.require();
			ui_provider.show_quick_pick(items_clone, options_clone).await
		})
	}))
}

/// Creates an effect to show an input box.
pub fn show_input_box(options:Option<InputBoxOptions>) -> ActionEffect<Arc<AppRuntime>, CommonError, Option<String>> {
	// Expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let options_clone = options.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let ui_provider:Arc<dyn UiProvider + Send + Sync> = concrete_env.require();
			ui_provider.show_input_box(options_clone).await
		})
	}))
}
