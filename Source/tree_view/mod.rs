

//
// @module tree_view
// @description This module defines the abstract contract for the Tree View service.
// It includes the `TreeViewProvider` trait, all related DTOs, and the `ActionEffect`
// constructors for every tree view operation.
//

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod TreeViewProvider;
pub use self::TreeViewProvider::TreeViewProvider;

// --- Data Transfer Objects ---
pub mod dto;

// --- Effect Constructors ---
mod RegisterTreeDataProvider;
mod UnregisterTreeDataProvider;
mod RevealTreeItem;
mod RefreshTreeView;
mod SetTreeViewMessage;
mod SetTreeViewTitle;
mod SetTreeViewBadge;

pub use self::RegisterTreeDataProvider::RegisterTreeDataProvider;
pub use self::UnregisterTreeDataProvider::UnregisterTreeDataProvider;
pub use self::RevealTreeItem::RevealTreeItem;
pub use self::RefreshTreeView::RefreshTreeView;
pub use self::SetTreeViewMessage::SetTreeViewMessage;
pub use self::SetTreeViewTitle::SetTreeViewTitle;
pub use self::SetTreeViewBadge::SetTreeViewBadge;
