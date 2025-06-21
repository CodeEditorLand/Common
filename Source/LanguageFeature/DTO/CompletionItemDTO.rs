//! # CompletionItemDTO
//!
//! Defines the Data Transfer Object for a single completion suggestion item.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct representing a single completion item, analogous to
/// `vscode.CompletionItem`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CompletionItemDTO {
	/// The label of this completion item.
	pub Label:Value, // Can be string or a CompletionItemLabel object

	/// The kind of this completion item.
	pub Kind:u32, // Corresponds to vscode.CompletionItemKind enum

	/// A human-readable string with additional information about this item.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Detail:Option<String>,

	/// A human-readable string that represents a doc-comment.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Documentation:Option<Value>, // string or IMarkdownStringDTO

	/// A string that should be used when comparing this item with other items.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SortText:Option<String>,

	/// A string that should be used when filtering a set of completion items.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FilterText:Option<String>,

	/// A string or snippet that should be inserted in a document when selecting
	/// this completion.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub InsertText:Option<Value>, // string or SnippetString DTO

	/// A range of text that should be replaced by this completion item.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Range:Option<Value>, // RangeDTO or { inserting: RangeDTO, replacing: RangeDTO }

	/// An optional array of additional text edits that are applied when
	/// selecting this completion.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub AdditionalTextEdits:Option<Vec<Value>>, // Vec<TextEditDTO>

	/// A command that should be executed after inserting this completion.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Command:Option<Value>, // CommandDTO
}
