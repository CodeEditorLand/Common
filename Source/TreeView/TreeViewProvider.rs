//! # TreeViewProvider Trait
//!
//! Defines the abstract service trait for creating and managing tree views
//! contributed by extensions.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};
// DTOs would be imported from a DTO module here.
// For now, they are defined inline or passed as `Value`.

/// An abstract service contract for an environment component that can manage
/// the lifecycle and data flow for custom tree views.
///
/// This trait is implemented by `MountainEnvironment`. It defines the
/// operations that `Cocoon` can request from the host to manage the UI state of
/// a tree view, which is then rendered by `Sky`.
#[async_trait]
pub trait TreeViewProvider: Environment + Send + Sync {
	/// Informs the host that a new tree data provider has been registered by
	/// an extension. The host should prepare to display a tree view with the
	/// given ID and options.
	///
	/// # Parameters
	/// * `ViewIdentifier`: A unique identifier for the tree view.
	/// * `Options`: DTO containing options like `CanSelectMany`.
	async fn RegisterTreeDataProvider(
		&self,
		ViewIdentifier:String,
		Options:Value, // TreeViewOptionsDTO
	) -> Result<(), CommonError>;

	/// Informs the host that a tree data provider has been disposed of.
	async fn UnregisterTreeDataProvider(&self, ViewIdentifier:String) -> Result<(), CommonError>;

	/// Asks the host UI to reveal and/or expand a specific item in a tree view.
	///
	/// # Parameters
	/// * `ViewIdentifier`: The ID of the target tree view.
	/// * `Item`: The DTO of the item to reveal.
	/// * `ParentChain`: An ordered list of parent items, from root to the
	///   item's immediate parent.
	/// * `Options`: DTO specifying reveal options like `select` and `focus`.
	async fn RevealTreeItem(
		&self,
		ViewIdentifier:String,
		Item:Value,             // TreeItemDTO
		ParentChain:Vec<Value>, // TreeItemDTO
		Options:Value,          // RevealOptionsDTO
	) -> Result<(), CommonError>;

	/// Notifies the host that some or all of the data in a tree view has
	/// changed and needs to be re-fetched and re-rendered.
	///
	/// # Parameters
	/// * `ViewIdentifier`: The ID of the tree view to refresh.
	/// * `ItemsToRefresh`: Optional. If `None`, the entire tree is refreshed.
	///   If `Some`, it's a DTO representing the specific items that have
	///   changed.
	async fn RefreshTreeView(
		&self,
		ViewIdentifier:String,
		ItemsToRefresh:Option<Value>, // DTO: Map<Handle, TreeItemDTO>
	) -> Result<(), CommonError>;

	/// Sets a message to be displayed in the tree view's empty state.
	async fn SetTreeViewMessage(&self, ViewIdentifier:String, Message:Value) -> Result<(), CommonError>;

	/// Sets the title and description of the tree view container.
	async fn SetTreeViewTitle(
		&self,
		ViewIdentifier:String,
		Title:String,
		Description:Option<String>,
	) -> Result<(), CommonError>;

	/// Sets a badge (e.g., a number) on the tree view's container icon.
	async fn SetTreeViewBadge(
		&self,
		ViewIdentifier:String,
		Badge:Option<Value>, // TreeViewBadgeDTO
	) -> Result<(), CommonError>;
}
