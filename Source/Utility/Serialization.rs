//! # Serialization Utilities
//!
//! Contains helper modules for custom `serde` serialization and deserialization
//! logic for types used in DTOs.

#![allow(non_snake_case, non_camel_case_types)]

/// A helper module for serializing and deserializing `url::Url` with `serde`.
/// This is used in DTOs where a `Url` field needs to be serialized to a string.
pub mod URLSerializationHelper {
	use serde::{self, Deserialize, Deserializer, Serializer};
	use url::Url;

	pub fn serialize<S>(URLInstance:&Url, SerializerInstance:S) -> Result<S::Ok, S::Error>
	where
		S: Serializer, {
		SerializerInstance.serialize_str(URLInstance.as_str())
	}

	pub fn deserialize<'de, D>(DeserializerInstance:D) -> Result<Url, D::Error>
	where
		D: Deserializer<'de>, {
		let StringValue = String::deserialize(DeserializerInstance)?;
		Url::parse(&StringValue).map_err(serde::de::Error::custom)
	}
}
