//! # CompletionContextDTO
//!
//! Defines the Data Transfer Object for the context of a completion request.

use serde::{Deserialize, Serialize};

/// Represents the reason why code completion was triggered.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionTriggerKindDTO {
	/// Completion was triggered by typing a trigger character.
	TriggerCharacter = 1,

	/// Completion was triggered explicitly by a command.
	Invoke = 2,
}

/// A serializable struct representing the context in which a completion was
/// requested, analogous to `vscode.CompletionContext`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CompletionContextDTO {
	/// The kind of trigger that initiated the completion request.
	pub TriggerKind:CompletionTriggerKindDTO,

	/// The character that triggered the completion request, if any.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TriggerCharacter:Option<String>,
}
