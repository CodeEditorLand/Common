// File: Common/Source/Keybinding/KeybindingProvider.rs
// Role: Defines the abstract service trait for resolving and providing
// keybindings. Responsibilities:
//   - Provide a contract for resolving the final, effective keymap by merging
//     default keybindings with user-defined ones from keybindings.json.

//! # KeybindingProvider Trait
//!
//! Defines the abstract service trait for resolving and providing keybindings.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can resolve
/// the final, effective keymap for the application by merging default and
/// user-defined keybindings.
#[async_trait]
pub trait KeybindingProvider: Environment + Send + Sync {
	/// Resolves and retrieves the complete list of active keybinding rules.
	///
	/// Reads default keybindings contributed by extensions, merges them with
	/// user-defined keybindings from `keybindings.json`, and returns the final
	/// list.
	///
	/// # Returns
	/// A `Result` containing a `Value` that is a JSON array of
	/// `KeybindingRuleDTO`s.
	async fn GetResolvedKeybinding(&self) -> Result<Value, CommonError>;
}
