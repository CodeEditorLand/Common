// File: Common/Source/TreeView/TreeViewProvider.rs
// Role: Defines the abstract service trait for creating and managing tree
// views. Responsibilities:
//   - Provide a contract for registering and unregistering tree data providers.
//   - Define the "push" operations extensions can use to control the view's
//     appearance (e.g., reveal, refresh, set title).
//   - Define the "pull" operations the host can use to fetch tree data on
//     demand (getChildren, getTreeItem).

//! # TreeViewProvider Trait
//!
//! Defines the abstract service trait for creating and managing tree views
//! contributed by extensions.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// the lifecycle and data flow for custom tree views.
/// Uses a hybrid model:
/// - "Push" methods are called by the extension host (`Cocoon`) to manage the
///   view's state and appearance in the main host (`Mountain`).
/// - "Pull" methods are called by the main host (`Mountain`) to request data
///   from the provider in the extension host (`Cocoon`) on-demand (e.g., when a
///   user expands a node).
#[async_trait]
pub trait TreeViewProvider: Environment + Send + Sync {
	// --- Methods called BY the extension host (PUSH) ---

	/// Informs the host that a new tree data provider has been registered by
	/// an extension. The host should prepare to display a tree view with the
	/// given ID and options.
	/// # Parameters
	/// * `ViewIdentifier`: A unique identifier for the tree view.
	/// * `OptionsValue`: DTO containing options like `CanSelectMany`.
	async fn RegisterTreeDataProvider(
		&self,

		ViewIdentifier:String,

		// DTO: TreeViewOptionsDTO
		OptionsValue:Value,
	) -> Result<(), CommonError>;

	/// Informs the host that a tree data provider has been disposed of.
	/// # Parameters
	/// * `ViewIdentifier`: The ID of the tree view to unregister.
	async fn UnregisterTreeDataProvider(&self, ViewIdentifier:String) -> Result<(), CommonError>;

	/// Asks the host UI to reveal and/or expand a specific item in a tree view.
	/// # Parameters
	/// * `ViewIdentifier`: The ID of the target tree view.
	/// * `ItemHandle`: The unique handle of the item to reveal.
	/// * `OptionsValue`: DTO specifying reveal options like `select` and
	///   `focus`.
	async fn RevealTreeItem(
		&self,

		ViewIdentifier:String,

		ItemHandle:String,

		// DTO: RevealOptionsDTO
		OptionsValue:Value,
	) -> Result<(), CommonError>;

	/// Notifies the host that some or all of the data in a tree view has
	/// changed and needs to be re-fetched and re-rendered.
	/// # Parameters
	/// * `ViewIdentifier`: The ID of the tree view to refresh.
	/// * `ItemsToRefreshValue`: Optional. If `None`, the entire tree is
	///   refreshed. If `Some`, it's a DTO representing the specific items that
	///   have changed, allowing for targeted updates.
	async fn RefreshTreeView(
		&self,

		ViewIdentifier:String,

		// DTO: Option<TreeItemDTO> or Vec<TreeItemDTO>
		ItemsToRefreshValue:Option<Value>,
	) -> Result<(), CommonError>;

	/// Sets a message to be displayed in the tree view's empty state.
	/// # Parameters
	/// * `ViewIdentifier`: The ID of the target tree view.
	/// * `Message`: An optional message to display. `None` clears the message.
	async fn SetTreeViewMessage(&self, ViewIdentifier:String, Message:Option<String>) -> Result<(), CommonError>;

	/// Sets the title and description of the tree view container.
	/// # Parameters
	/// * `ViewIdentifier`: The ID of the target tree view.
	/// * `Title`: An optional new title for the view. `None` might reset to
	///   default.
	/// * `Description`: An optional description or sub-title. `None` clears it.
	async fn SetTreeViewTitle(
		&self,

		ViewIdentifier:String,

		Title:Option<String>,

		Description:Option<String>,
	) -> Result<(), CommonError>;

	/// Sets a badge (e.g., a number) on the tree view's container icon.
	/// # Parameters
	/// * `ViewIdentifier`: The ID of the target tree view.
	/// * `BadgeValue`: A DTO for the badge, or `None` to clear it.
	async fn SetTreeViewBadge(
		&self,

		ViewIdentifier:String,

		// DTO: TreeViewBadgeDTO
		BadgeValue:Option<Value>,
	) -> Result<(), CommonError>;

	// --- Methods called BY the host (PULL) ---

	/// Retrieves the children for a given tree element.
	/// # Parameters
	/// * `ViewIdentifier`: The ID of the target tree view.
	/// * `ElementHandle`: An optional handle for the parent element. If `None`,

	///   the root-level items should be returned.
	/// # Returns
	/// A vector of `TreeItemDTO`s representing the children.
	async fn GetChildren(&self, ViewIdentifier:String, ElementHandle:Option<String>)
	-> Result<Vec<Value>, CommonError>;

	/// Retrieves the full `TreeItem` DTO for a given element handle.
	/// This is used by the host to get the display properties of an item.
	/// # Parameters
	/// * `ViewIdentifier`: The ID of the target tree view.
	/// * `ElementHandle`: The handle of the element to retrieve.
	/// # Returns
	/// A `TreeItemDTO` as a JSON value.
	async fn GetTreeItem(&self, ViewIdentifier:String, ElementHandle:String) -> Result<Value, CommonError>;

	// --- State Management Methods ---

	/// Handles tree node expansion/collapse events.
	/// Called when a user expands or collapses a node in the tree view.
	/// # Parameters
	/// * `ViewIdentifier`: The ID of the target tree view.
	/// * `ElementHandle`: The handle of the element being expanded/collapsed.
	/// * `IsExpanded`: Whether the node is being expanded (true) or collapsed
	///   (false).
	async fn OnTreeNodeExpanded(
		&self,

		ViewIdentifier:String,

		ElementHandle:String,

		IsExpanded:bool,
	) -> Result<(), CommonError>;

	/// Handles tree selection changes.
	/// Called when the user selects or deselects items in the tree view.
	/// # Parameters
	/// * `ViewIdentifier`: The ID of the target tree view.
	/// * `SelectedHandles`: Vector of handles currently selected.
	async fn OnTreeSelectionChanged(
		&self,

		ViewIdentifier:String,

		SelectedHandles:Vec<String>,
	) -> Result<(), CommonError>;

	/// Persists the current state of a tree view.
	/// Saves the expansion and selection state for later restoration.
	/// # Parameters
	/// * `ViewIdentifier`: The ID of the tree view to persist.
	/// # Returns
	/// JSON value representing the persisted state.
	async fn PersistTreeViewState(&self, ViewIdentifier:String) -> Result<Value, CommonError>;

	/// Restores a previously persisted tree view state.
	/// # Parameters
	/// * `ViewIdentifier`: The ID of the tree view to restore.
	/// * `StateValue`: The previously persisted state as JSON value.
	async fn RestoreTreeViewState(&self, ViewIdentifier:String, StateValue:Value) -> Result<(), CommonError>;
}
