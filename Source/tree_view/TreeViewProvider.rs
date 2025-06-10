use async_trait::async_trait;
use serde_json::Value;

/// @module TreeViewProvider
/// @description Defines the abstract service trait for creating and managing
/// tree views contributed by extensions.
use super::dto::*;
use crate::{environment::Environment, error::CommonError};

/// An abstract service contract for an environment component that can manage
/// the lifecycle and data flow for tree views.
///
/// This trait is implemented by `MountainEnvironment`. It defines the
/// operations that `Cocoon` can request from the host to manage the UI state of
/// a tree view.
#[async_trait]
pub trait TreeViewProvider: Environment + Send + Sync {
	/// Informs the host that a new tree data provider has been registered by an
	/// extension. The host should prepare to display a tree view with the
	/// given ID and options.
	///
	/// @param ViewId - A unique identifier for the tree view.
	/// @param Options - DTO containing options like `canSelectMany`.
	async fn RegisterTreeDataProvider(&self, ViewId:String, Options:TreeViewOptionsDto) -> Result<(), CommonError>;

	/// Informs the host that a tree data provider has been disposed of.
	async fn UnregisterTreeDataProvider(&self, ViewId:String) -> Result<(), CommonError>;

	/// Asks the host UI to reveal and/or expand a specific item in a tree view.
	///
	/// @param ViewId - The ID of the target tree view.
	/// @param Item - The DTO of the item to reveal.
	/// @param ParentChain - An ordered list of parent items, from root to
	/// immediate parent. @param Options - DTO specifying reveal options like
	/// `select` and `focus`.
	async fn RevealTreeItem(
		&self,
		ViewId:String,
		Item:TreeItemDto,
		ParentChain:Vec<TreeItemDto>,
		Options:RevealOptionsDto,
	) -> Result<(), CommonError>;

	/// Notifies the host that some or all of the data in a tree view has
	/// changed and needs to be re-fetched and re-rendered.
	///
	/// @param ViewId - The ID of the tree view to refresh.
	/// @param ItemsToRefresh - Optional. If `None`, the entire tree is
	/// refreshed.   If `Some`, it's a DTO representing the specific items that
	/// have changed.
	async fn RefreshTreeView(
		&self,
		ViewId:String,
		ItemsToRefresh:Option<Value>, // DTO: Map<Handle, TreeItemDto>
	) -> Result<(), CommonError>;

	/// Sets a message to be displayed in the tree view's empty state.
	async fn SetTreeViewMessage(&self, ViewId:String, Message:Value) -> Result<(), CommonError>;

	/// Sets the title and description of the tree view container.
	async fn SetTreeViewTitle(
		&self,
		ViewId:String,
		Title:String,
		Description:Option<String>,
	) -> Result<(), CommonError>;

	/// Sets a badge (e.g., a number) on the tree view's container icon.
	async fn SetTreeViewBadge(&self, ViewId:String, Badge:Option<TreeViewBadgeDto>) -> Result<(), CommonError>;
}
