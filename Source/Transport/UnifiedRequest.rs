#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
//! # UnifiedRequest
//!
//! A protocol-agnostic request message that works across all transport types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::Common::{
	CorrelationId, SystemTimestampGenerator, Timestamp, TimestampGenerator, TransportType,
	UuidCorrelationIdGenerator, CorrelationIdGenerator,
};

/// A unified request message that can be sent over any transport.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnifiedRequest {
	/// Unique correlation ID for request/response matching.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub CorrelationIdentifier: Option<CorrelationId>,

	/// The method to invoke, using dot notation (e.g., "fileSystem.readFile").
	pub Method: String,

	/// Binary payload containing serialized parameters for the method.
	pub Payload: Vec<u8>,

	/// Optional metadata for the request.
	#[serde(skip_serializing_if = "HashMap::is_empty")]
	pub Metadata: HashMap<String, String>,

	/// Timestamp when the request was created (microseconds since Unix epoch).
	pub CreatedAt: Timestamp,

	/// Optional hint for preferred transport type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub TransportHint: Option<TransportType>,
}

impl UnifiedRequest {
	/// Creates a new `UnifiedRequest` with the given method.
	pub fn New(Method: impl Into<String>) -> Self {
		Self {
			CorrelationIdentifier: Some(UuidCorrelationIdGenerator::Generate()),
			Method: Method.into(),
			Payload: Vec::new(),
			Metadata: HashMap::new(),
			CreatedAt: SystemTimestampGenerator::Now(),
			TransportHint: None,
		}
	}

	/// Sets the correlation ID explicitly.
	pub fn WithCorrelationIdentifier(mut self, CorrelationIdentifier: CorrelationId) -> Self {
		self.CorrelationIdentifier = Some(CorrelationIdentifier);
		self
	}

	/// Sets the binary payload.
	pub fn WithPayload(mut self, Payload: Vec<u8>) -> Self {
		self.Payload = Payload;
		self
	}

	/// Adds a metadata key-value pair.
	pub fn WithMetadata(
		mut self,
		Key: impl Into<String>,
		Value: impl Into<String>,
	) -> Self {
		self.Metadata.insert(Key.into(), Value.into());
		self
	}

	/// Sets the entire metadata map.
	pub fn WithMetadataMap(mut self, Metadata: HashMap<String, String>) -> Self {
		self.Metadata = Metadata;
		self
	}

	/// Sets the request timeout in milliseconds.
	pub fn WithTimeout(mut self, TimeoutMilliseconds: u64) -> Self {
		self.Metadata
			.insert("timeout_ms".to_string(), TimeoutMilliseconds.to_string());
		self
	}

	/// Sets the request priority.
	pub fn WithPriority(mut self, Priority: u32) -> Self {
		self.Metadata
			.insert("priority".to_string(), Priority.to_string());
		self
	}

	/// Sets the preferred transport type.
	pub fn WithTransportHint(mut self, TransportKind: TransportType) -> Self {
		self.TransportHint = Some(TransportKind);
		self
	}

	/// Gets the timeout from metadata, if present.
	pub fn TimeoutMilliseconds(&self) -> Option<u64> {
		self.Metadata.get("timeout_ms").and_then(|Value| Value.parse().ok())
	}

	/// Gets the priority from metadata, if present.
	pub fn Priority(&self) -> Option<u32> {
		self.Metadata.get("priority").and_then(|Value| Value.parse().ok())
	}

	/// Validates the request.
	pub fn Validate(&self) -> Result<(), String> {
		if self.Method.is_empty() {
			return Err("method cannot be empty".to_string());
		}

		if let Some(Identifier) = &self.CorrelationIdentifier {
			if Identifier.is_empty() {
				return Err("correlation_id cannot be empty if specified".to_string());
			}
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn TestUnifiedRequestCreation() {
		let Request = UnifiedRequest::New("test.method");
		assert!(!Request.Method.is_empty());
		assert!(Request.CorrelationIdentifier.is_some());
		assert_eq!(Request.Payload, Vec::new());
		assert!(Request.Metadata.is_empty());
		assert!(Request.TransportHint.is_none());
	}

	#[test]
	fn TestUnifiedRequestBuilder() {
		let Request = UnifiedRequest::New("fileSystem.readFile")
			.WithPayload(b"{\"path\": \"/tmp/test.txt\"}".to_vec())
			.WithTimeout(5000)
			.WithPriority(10)
			.WithMetadata(
				"traceparent",
				"00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01",
			)
			.WithTransportHint(TransportType::Grpc);

		assert_eq!(Request.Method, "fileSystem.readFile");
		assert_eq!(Request.Payload, b"{\"path\": \"/tmp/test.txt\"}");
		assert_eq!(Request.TimeoutMilliseconds(), Some(5000));
		assert_eq!(Request.Priority(), Some(10));
		assert_eq!(Request.TransportHint, Some(TransportType::Grpc));
		assert!(Request.Metadata.contains_key("traceparent"));
	}

	#[test]
	fn TestUnifiedRequestValidation() {
		let mut Request = UnifiedRequest::New("valid.method");
		assert!(Request.Validate().is_ok());

		Request.Method = String::new();
		assert!(Request.Validate().is_err());

		Request.Method = "valid.method".to_string();
		Request.CorrelationIdentifier = Some("".to_string());
		assert!(Request.Validate().is_err());
	}
}
