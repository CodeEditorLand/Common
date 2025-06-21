//! # LocationDTO
//!
//! Defines the Data Transfer Object for representing a location, which includes
//! a URI and a range within that resource.

use serde::{Deserialize, Serialize};
use url::Url;

use super::RangeDTO::RangeDTO;
use crate::Utility::Serialization::URLSerializationHelper;

/// A serializable struct representing a location, analogous to
/// `vscode.Location`. It is a core building block for features like "Go to
/// Definition" and "Find All References".
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LocationDTO {
	/// The URI of the resource.
	#[serde(with = "URLSerializationHelper")]
	pub Uri:Url,

	/// The range within the resource.
	pub Range:RangeDTO,
}
