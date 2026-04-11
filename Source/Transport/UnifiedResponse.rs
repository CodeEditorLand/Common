#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
//! # UnifiedResponse
//!
//! A protocol-agnostic response message that works across all transport types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
	Common::{CorrelationId, SystemTimestampGenerator, Timestamp, TimestampGenerator},
	TransportStrategy::TransportErrorCode,
};

/// A unified response message that can be received over any transport.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnifiedResponse {
	/// Correlation ID matching the request.
	pub CorrelationIdentifier:CorrelationId,

	/// Success flag indicating whether the operation completed successfully.
	pub Success:bool,

	/// Binary payload containing the serialized result (if `Success = true`).
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub Payload:Vec<u8>,

	/// Error information when `Success = false`.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub Error:Option<ResponseError>,

	/// Additional response metadata.
	#[serde(skip_serializing_if = "HashMap::is_empty")]
	pub Metadata:HashMap<String, String>,

	/// Timestamp when the response was generated (microseconds since Unix
	/// epoch).
	pub GeneratedAt:Timestamp,
}

impl UnifiedResponse {
	/// Creates a new successful response with the given correlation ID and
	/// payload.
	pub fn Success(CorrelationIdentifier:CorrelationId, Payload:Vec<u8>) -> Self {
		Self {
			CorrelationIdentifier,
			Success:true,
			Payload,
			Error:None,
			Metadata:HashMap::new(),
			GeneratedAt:SystemTimestampGenerator::Now(),
		}
	}

	/// Creates a new error response with the given correlation ID and error.
	pub fn Failure(CorrelationIdentifier:CorrelationId, Error:ResponseError, Payload:Option<Vec<u8>>) -> Self {
		Self {
			CorrelationIdentifier,
			Success:false,
			Payload:Payload.unwrap_or_default(),
			Error:Some(Error),
			Metadata:HashMap::new(),
			GeneratedAt:SystemTimestampGenerator::Now(),
		}
	}

	/// Creates a new error response from a `TransportError`.
	pub fn FromTransportError(
		CorrelationIdentifier:CorrelationId,
		TransportError:&super::TransportError::TransportError,
	) -> Self {
		Self::Failure(
			CorrelationIdentifier,
			ResponseError {
				Code:TransportError.Code,
				Message:TransportError.Message.clone(),
				Details:TransportError.Context.clone(),
			},
			None,
		)
	}

	/// Adds metadata to the response.
	pub fn WithMetadata(mut self, Key:impl Into<String>, Value:impl Into<String>) -> Self {
		self.Metadata.insert(Key.into(), Value.into());
		self
	}

	/// Sets the entire metadata map.
	pub fn WithMetadataMap(mut self, Metadata:HashMap<String, String>) -> Self {
		self.Metadata = Metadata;
		self
	}

	/// Gets the error code if this is an error response.
	pub fn ErrorCode(&self) -> Option<TransportErrorCode> { self.Error.as_ref().map(|ErrorInfo| ErrorInfo.Code) }

	/// Checks if this response is a success.
	pub fn IsSuccess(&self) -> bool { self.Success }

	/// Checks if this response is an error.
	pub fn IsError(&self) -> bool { !self.Success }

	/// Validates the response.
	pub fn Validate(&self) -> Result<(), String> {
		if self.CorrelationIdentifier.is_empty() {
			return Err("correlation_id cannot be empty".to_string());
		}

		if self.Success && self.Error.is_some() {
			return Err("success response must not have error".to_string());
		}

		if !self.Success && self.Error.is_none() {
			return Err("error response must have error field".to_string());
		}

		Ok(())
	}
}

/// Error information within a response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseError {
	/// Error code indicating the failure type.
	pub Code:TransportErrorCode,

	/// Human-readable error message.
	pub Message:String,

	/// Optional additional details as key-value pairs.
	#[serde(skip_serializing_if = "HashMap::is_empty")]
	pub Details:HashMap<String, String>,
}

impl ResponseError {
	/// Creates a new `ResponseError` with the given code and message.
	pub fn New(Code:TransportErrorCode, Message:impl Into<String>) -> Self {
		Self { Code, Message:Message.into(), Details:HashMap::new() }
	}

	/// Adds a detail key-value pair to the error.
	pub fn WithDetail(mut self, Key:impl Into<String>, Value:impl Into<String>) -> Self {
		self.Details.insert(Key.into(), Value.into());
		self
	}
}

impl std::fmt::Display for ResponseError {
	fn fmt(&self, Formatter:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(Formatter, "{} (code: {:?})", self.Message, self.Code)?;
		if !self.Details.is_empty() {
			let DetailsString:Vec<String> =
				self.Details.iter().map(|(Key, Value)| format!("{}={}", Key, Value)).collect();
			write!(Formatter, " [{}]", DetailsString.join(", "))?;
		}
		Ok(())
	}
}

impl std::error::Error for ResponseError {}

#[cfg(test)]
mod tests {
	use TransportErrorCode::ConnectionFailed;

	use super::*;
	use crate::Transport::TransportStrategy::TransportErrorCode;

	#[test]
	fn TestUnifiedResponseSuccess() {
		let Response = UnifiedResponse::Success("req-123".to_string(), b"result".to_vec());
		assert!(Response.Success);
		assert_eq!(Response.CorrelationIdentifier, "req-123");
		assert_eq!(Response.Payload, b"result");
		assert!(Response.Error.is_none());
	}

	#[test]
	fn TestUnifiedResponseError() {
		let Error = ResponseError::New(ConnectionFailed, "Connection timeout");
		let Response = UnifiedResponse::Failure("req-456".to_string(), Error, None);

		assert!(!Response.Success);
		assert_eq!(Response.CorrelationIdentifier, "req-456");
		assert!(Response.Error.is_some());
		assert_eq!(Response.Error.as_ref().unwrap().Code, ConnectionFailed);
	}

	#[test]
	fn TestUnifiedResponseFromTransportError() {
		let TransportErrorValue = super::super::TransportError::TransportError::New(ConnectionFailed, "Conn failed")
			.WithMethod("test.method");
		let Response = UnifiedResponse::FromTransportError("req-789".to_string(), &TransportErrorValue);

		assert!(!Response.Success);
		assert_eq!(Response.Error.as_ref().unwrap().Code, ConnectionFailed);
		assert!(Response.Error.as_ref().unwrap().Message.contains("Conn failed"));
	}

	#[test]
	fn TestResponseValidation() {
		let Response = UnifiedResponse::Success("abc".to_string(), Vec::new());
		assert!(Response.Validate().is_ok());

		let mut Invalid = Response.clone();
		Invalid.CorrelationIdentifier = String::new();
		assert!(Invalid.Validate().is_err());

		let mut Invalid2 = Response.clone();
		Invalid2.Success = false;
		Invalid2.Error = None;
		assert!(Invalid2.Validate().is_err());
	}
}
