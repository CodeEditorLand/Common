//! # SourceControlCreateDTO
//!
//! Defines the DTO for creating a new Source Control provider.

use serde::{Deserialize, Serialize};
use url::Url;

use crate::Utility::Serialization::URLSerializationHelper;

/// A serializable struct sent from the extension host (`Cocoon`) to the main
/// host (`Mountain`) when an extension calls `vscode.scm.createSourceControl`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SourceControlCreateDTO {
	/// The unique identifier for this source control provider.
	pub ID:String,
	/// The human-readable label for this provider (e.g., "Git").
	pub Label:String,
	/// The root URI of the repository this provider is managing.
	#[serde(with = "URLSerializationHelper")]
	pub RootUri:Url,
}
