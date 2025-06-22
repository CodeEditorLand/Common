//! # WorkSpaceEditDTO
//!
//! Defines the Data Transfer Object for a workspace edit, which is a collection
//! of changes to be applied across multiple files in the workspace.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable struct representing a workspace edit, which is a batch of
/// changes that can include text edits and file operations (create, delete,
/// rename). This is analogous to `vscode.WorkspaceEdit`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct WorkSpaceEditDTO {
	/// A list of text edits to apply, grouped by resource URI.
	/// The structure is `[ [uri_dto, [text_edit_dto, ...]], ... ]`
	pub Edits:Vec<(Value, Vec<Value>)>,
}
