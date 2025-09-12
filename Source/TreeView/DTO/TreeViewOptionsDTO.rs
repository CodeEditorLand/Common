//! # TreeViewOptionsDTO
//!
//! Defines the Data Transfer Object for a tree view's configuration options.

#![allow(non_snake_case, non_camel_case_types)]

use serde::{Deserialize, Serialize};

/// A struct that holds the configuration options for a tree view instance
/// registered by an extension.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TreeViewOptionsDTO {
	#[serde(default)]
	pub CanSelectMany:bool,

	#[serde(default)]
	pub HasHandleDrag:bool,

	#[serde(default)]
	pub HasHandleDrop:bool,
}
