//! # UnifiedResponse
//!
//! A protocol-agnostic response message that works across all transport types.
//!
//! This DTO provides a uniform way to receive responses regardless of the
//! underlying transport mechanism (gRPC, IPC, WASM). It represents the result
//! of a request or a notification acknowledgment.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    Common::CorrelationId,
    TransportError::TransportErrorCode,
};

/// A unified response message that can be received over any transport.
///
/// This is the standard response format used by all CodeEditorLand components.
/// Responses correlate to a request via `correlation_id` and contain either
/// a successful result or an error description.
///
/// # Structure
///
/// The response consists of:
///
/// - `correlation_id`: Matches the request's correlation ID for correlation
/// - `success`: Boolean indicating whether the operation succeeded
/// - `payload`: Binary payload containing the serialized result (if success)
/// - `error`: Optional error information (if failure)
/// - `metadata`: Additional response metadata (headers, tracing, etc.)
/// - `timestamp`: When the response was generated (microseconds since epoch)
///
/// # Success vs Failure
///
/// - **Success**: `success = true`, `payload` contains result, `error = None`
/// - **Failure**: `success = false`, `payload` may be empty or contain error details,
///   `error` is `Some(ResponseError)`
///
/// The `payload` on failure is optional and transport-specific. Typically it's
/// empty, but some transports may include additional error context.
///
/// # Example
///
/// ```rust,ignore
/// let response: UnifiedResponse = transport.send_request(request).await?;
///
/// if response.success {
///     let result: FileReadResult = serde_json::from_slice(&response.payload)?;
///     // Use result...
/// } else {
///     let error = response.error.unwrap();
///     eprintln!("Request failed: {} (code: {:?})", error.message, error.code);
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnifiedResponse {
    /// Correlation ID matching the request.
    /// This must be present and should equal the request's correlation_id.
    pub correlation_id: CorrelationId,

    /// Success flag indicating whether the operation completed successfully.
    /// If `false`, the `error` field must be populated.
    pub success: bool,

    /// Binary payload containing the serialized result (if `success = true`).
    /// The format depends on the method's return type (JSON, protobuf, raw bytes).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<u8>,

    /// Error information when `success = false`.
    /// Must be `None` when `success = true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,

    /// Additional response metadata (headers, tracing info, etc.).
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,

    /// Timestamp when the response was generated (microseconds since Unix epoch).
    /// Used for latency calculation and debugging.
    pub timestamp: Timestamp,
}

impl UnifiedResponse {
    /// Creates a new successful response with the given correlation ID and payload.
    ///
    /// # Parameters
    ///
    /// * `correlation_id`: The correlation ID from the request
    /// * `payload`: Serialized result data
    ///
    /// # Returns
    ///
    /// A `UnifiedResponse` with `success = true` and no error.
    pub fn success(correlation_id: CorrelationId, payload: Vec<u8>) -> Self {
        Self {
            correlation_id,
            success: true,
            payload,
            error: None,
            metadata: HashMap::new(),
            timestamp: Timestamp::now(),
        }
    }

    /// Creates a new error response with the given correlation ID and error.
    ///
    /// # Parameters
    ///
    /// * `correlation_id`: The correlation ID from the request
    /// * `error`: The error that occurred
    /// * `payload`: Optional payload with additional error context (used for debugging)
    ///
    /// # Returns
    ///
    /// A `UnifiedResponse` with `success = false` and the error populated.
    pub fn error(correlation_id: CorrelationId, error: ResponseError, payload: Option<Vec<u8>>) -> Self {
        Self {
            correlation_id,
            success: false,
            payload: payload.unwrap_or_default(),
            error: Some(error),
            metadata: HashMap::new(),
            timestamp: Timestamp::now(),
        }
    }

    /// Creates a new error response from a `TransportError`.
    ///
    /// This convenience constructor maps a `TransportError` to a `ResponseError`
    /// with appropriate code and message.
    pub fn from_transport_error(correlation_id: CorrelationId, transport_error: &super::TransportError) -> Self {
        Self::error(
            correlation_id,
            ResponseError {
                code: transport_error.code,
                message: transport_error.message.clone(),
                details: transport_error.context.clone(),
            },
            None,
        )
    }

    /// Adds metadata to the response.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Sets the entire metadata map.
    pub fn with_metadata_map(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }

    /// Gets the error code if this is an error response.
    pub fn error_code(&self) -> Option<TransportErrorCode> {
        self.error.as_ref().map(|e| e.code)
    }

    /// Checks if this response is a success.
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Checks if this response is an error.
    pub fn is_error(&self) -> bool {
        !self.success
    }

    /// Validates the response.
    ///
    /// Ensures that:
    /// - correlation_id is not empty
    /// - success flag consistency (error present iff success = false)
    ///
    /// Returns `Ok(())` if valid, or an error string if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.correlation_id.is_empty() {
            return Err("correlation_id cannot be empty".to_string());
        }

        if self.success && self.error.is_some() {
            return Err("success response must not have error".to_string());
        }

        if !self.success && self.error.is_none() {
            return Err("error response must have error field".to_string());
        }

        Ok(())
    }
}

/// Error information within a response.
///
/// This structure provides structured error data when a request fails.
/// It includes a standardized error code, human-readable message, and
/// optional key-value details for debugging.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseError {
    /// Error code indicating the failure type.
    /// These codes are transport-agnostic and map to standard failure modes.
    pub code: TransportErrorCode,

    /// Human-readable error message.
    pub message: String,

    /// Optional additional details as key-value pairs.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub details: HashMap<String, String>,
}

impl ResponseError {
    /// Creates a new `ResponseError` with the given code and message.
    pub fn new(code: TransportErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: HashMap::new(),
        }
    }

    /// Adds a detail key-value pair to the error.
    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }
}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (code: {:?})", self.message, self.code)?;
        if !self.details.is_empty() {
            let details_str: Vec<String> = self.details.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
            write!(f, " [{}]", details_str.join(", "))?;
        }
        Ok(())
    }
}

impl std::error::Error for ResponseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Transport::TransportErrorCode::ConnectionFailed;

    #[test]
    fn test_unified_response_success() {
        let response = UnifiedResponse::success("req-123".to_string(), b"result".to_vec());
        assert!(response.success);
        assert_eq!(response.correlation_id, "req-123");
        assert_eq!(response.payload, b"result");
        assert!(response.error.is_none());
    }

    #[test]
    fn test_unified_response_error() {
        let error = ResponseError::new(ConnectionFailed, "Connection timeout");
        let response = UnifiedResponse::error("req-456".to_string(), error, None);

        assert!(!response.success);
        assert_eq!(response.correlation_id, "req-456");
        assert!(response.error.is_some());
        assert_eq!(response.error.as_ref().unwrap().code, ConnectionFailed);
    }

    #[test]
    fn test_unified_response_from_transport_error() {
        let transport_error = super::super::TransportError::new(ConnectionFailed, "Conn failed")
            .with_method("test.method");
        let response = UnifiedResponse::from_transport_error("req-789".to_string(), &transport_error);

        assert!(!response.success);
        assert_eq!(response.error.as_ref().unwrap().code, ConnectionFailed);
        assert!(response.error.as_ref().unwrap().message.contains("Conn failed"));
    }

    #[test]
    fn test_response_validation() {
        let response = UnifiedResponse::success("abc".to_string(), Vec::new());
        assert!(response.validate().is_ok());

        let mut invalid = response.clone();
        invalid.correlation_id = String::new();
        assert!(invalid.validate().is_err());

        let mut invalid2 = response.clone();
        invalid2.success = false;
        invalid2.error = None;
        assert!(invalid2.validate().is_err());
    }
}
