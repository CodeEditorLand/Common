//! # CompletionItemDTO
//!
//! Defines the Data Transfer Object for a single completion suggestion item.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct representing a single completion item, analogous to
/// `vscode.CompletionItem`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItemDTO {
	/// The label of this completion item.
	// Can be string or a CompletionItemLabel object
	pub Label: Value,

	/// The kind of this completion item.
	// Corresponds to vscode.CompletionItemKind enum
	pub Kind: u32,

	/// A human-readable string with additional information about this item.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Detail:Option<String>,

	/// A human-readable string that represents a doc-comment.
	#[serde(skip_serializing_if = "Option::is_none")]
	// string or IMarkdownStringDTO
	pub Documentation: Option<Value>,

	/// A string that should be used when comparing this item with other items.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub SortText:Option<String>,

	/// A string that should be used when filtering a set of completion items.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub FilterText:Option<String>,

	/// A string or snippet that should be inserted in a document when selecting
	/// this completion.
	#[serde(skip_serializing_if = "Option::is_none")]
	// string or SnippetString DTO
	pub InsertText: Option<Value>,

	/// A range of text that should be replaced by this completion item.
	#[serde(skip_serializing_if = "Option::is_none")]
	// RangeDTO or { inserting: RangeDTO, replacing: RangeDTO }
	pub Range: Option<Value>,

	/// An optional array of additional text edits that are applied when
	/// selecting this completion.
	#[serde(skip_serializing_if = "Option::is_none")]
	// Vec<TextEditDTO>
	pub AdditionalTextEdits: Option<Vec<Value>>,

	/// A command that should be executed after inserting this completion.
	#[serde(skip_serializing_if = "Option::is_none")]
	// CommandDTO
	pub Command: Option<Value>,
}
