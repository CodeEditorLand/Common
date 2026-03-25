//! # UnifiedRequest
//!
//! A protocol-agnostic request message that works across all transport types.
//!
//! This DTO provides a uniform way to send requests regardless of the underlying
//! transport mechanism (gRPC, IPC, WASM). It encapsulates method invocation
//! with parameters, correlation tracking, and metadata.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::Common::{
    CorrelationId, CorrelationIdGenerator, Timestamp, TimestampGenerator, TransportType,
};

/// A unified request message that can be sent over any transport.
///
/// This is the standard request format used by all CodeEditorLand components
/// when making RPC-style calls. It abstracts away the transport-specific details
/// and provides a consistent API for method invocation.
///
/// # Structure
///
/// The request consists of:
///
/// - `correlation_id`: Unique ID for request/response matching (auto-generated if None)
/// - `method`: The method name to invoke (e.g., "fileSystem.readFile")
/// - `payload`: Binary payload containing serialized parameters (usually JSON or protobuf)
/// - `metadata`: Key-value pairs for additional context (timeout, priority, etc.)
/// - `timestamp`: When the request was created (microseconds since epoch)
/// - `transport_hint`: Optional suggestion for which transport to use
///
/// # Serialization
///
/// The `payload` field contains the serialized parameters. The serialization
/// format depends on the method being called:
///
/// - JSON for most RPC calls (readable, debuggable)
/// - Protocol Buffers for performance-critical paths
/// - Raw bytes for binary data (file content, images, etc.)
///
/// The payload should be deserialized according to the method's expected
/// parameter type. The response will contain a similarly serialized result.
///
/// # Example
///
/// ```rust,ignore
/// use common_common::transport::UnifiedRequest;
///
/// let request = UnifiedRequest::new("fileSystem.readFile")
///     .with_payload(serde_json::to_vec(&params).unwrap())
///     .with_timeout(5000)
///     .with_priority(10);
///
/// let response = transport.send_request(request).await?;
/// let result: FileReadResult = serde_json::from_slice(&response.payload)?;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnifiedRequest {
    /// Unique correlation ID for request/response matching.
    /// If `None`, the transport will generate a UUID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<CorrelationId>,

    /// The method to invoke, using dot notation (e.g., "fileSystem.readFile").
    /// This should be globally unique across all services.
    pub method: String,

    /// Binary payload containing serialized parameters for the method.
    /// The format (JSON, protobuf, etc.) is determined by the method's contract.
    pub payload: Vec<u8>,

    /// Optional metadata for the request.
    /// Common keys: "timeout_ms", "priority", "compression", "routing_key"
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,

    /// Timestamp when the request was created (microseconds since Unix epoch).
    /// Set by the sender; used for latency measurement and timeout calculation.
    pub timestamp: Timestamp,

    /// Optional hint for preferred transport type.
    /// The transport layer may ignore this if the requested transport is unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_hint: Option<TransportType>,
}

impl UnifiedRequest {
    /// Creates a new `UnifiedRequest` with the given method.
    ///
    /// This constructor automatically generates:
    /// - A new correlation ID (UUID v4)
    /// - Current timestamp
    /// - Empty payload
    /// - Empty metadata
    ///
    /// # Parameters
    ///
    /// * `method`: The method name (e.g., "fileSystem.readFile")
    ///
    /// # Returns
    ///
    /// A new `UnifiedRequest` with default values.
    pub fn new(method: impl Into<String>) -> Self {
        Self {
            correlation_id: Some(CorrelationId::generate()),
            method: method.into(),
            payload: Vec::new(),
            metadata: HashMap::new(),
            timestamp: Timestamp::now(),
            transport_hint: None,
        }
    }

    /// Sets the correlation ID explicitly.
    ///
    /// Normally you'd let the transport generate this, but sometimes you
    /// need to propagate a correlation ID from an upstream request.
    pub fn with_correlation_id(mut self, correlation_id: CorrelationId) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    /// Sets the binary payload.
    ///
    /// The payload should be the serialized parameters for the method.
    /// Use `serde_json::to_vec(&params)` for JSON or `prost::Message::encode_to_vec`
    /// for protobuf.
    pub fn with_payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = payload;
        self
    }

    /// Adds a metadata key-value pair.
    ///
    /// Metadata is used for routing, timeouts, priorities, and other
    /// transport-specific or application-specific information.
    ///
    /// Common metadata keys:
    /// - `timeout_ms`: Request timeout in milliseconds (overrides transport default)
    /// - `priority`: Numeric priority (higher = more important)
    /// - `compression`: "gzip", "zstd", or "none"
    /// - `routing_key`: Arbitrary key for transport routing decisions
    /// - `traceparent`: W3C trace context for distributed tracing
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Sets the entire metadata map.
    pub fn with_metadata_map(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }

    /// Sets the request timeout in milliseconds.
    ///
    /// This is a convenience method that adds `timeout_ms` to metadata.
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.metadata.insert("timeout_ms".to_string(), timeout_ms.to_string());
        self
    }

    /// Sets the request priority.
    ///
    /// Higher numbers indicate higher priority. Priority interpretation
    /// is transport-specific but typically affects queuing and resource allocation.
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.metadata.insert("priority".to_string(), priority.to_string());
        self
    }

    /// Sets the preferred transport type.
    ///
    /// The transport layer will attempt to use this transport if available.
    /// If it's not available or fails, it may fall back to automatic selection.
    pub fn with_transport_hint(mut self, transport_type: TransportType) -> Self {
        self.transport_hint = Some(transport_type);
        self
    }

    /// Gets the timeout from metadata, if present.
    ///
    /// Returns the timeout in milliseconds, or `None` if not specified.
    pub fn timeout_ms(&self) -> Option<u64> {
        self.metadata.get("timeout_ms").and_then(|s| s.parse().ok())
    }

    /// Gets the priority from metadata, if present.
    pub fn priority(&self) -> Option<u32> {
        self.metadata.get("priority").and_then(|s| s.parse().ok())
    }

    /// Validates the request.
    ///
    /// Checks that required fields are present and values are sensible.
    /// Returns `Ok(())` if valid, or an error string if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.method.is_empty() {
            return Err("method cannot be empty".to_string());
        }

        // Correlation ID can be None (transport will generate), but if present must be non-empty
        if let Some(id) = &self.correlation_id {
            if id.is_empty() {
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
    fn test_unified_request_creation() {
        let request = UnifiedRequest::new("test.method");
        assert!(!request.method.is_empty());
        assert!(request.correlation_id.is_some());
        assert_eq!(request.payload, Vec::new());
        assert!(request.metadata.is_empty());
        assert!(request.transport_hint.is_none());
    }

    #[test]
    fn test_unified_request_builder() {
        let request = UnifiedRequest::new("fileSystem.readFile")
            .with_payload(b"{\"path\": \"/tmp/test.txt\"}".to_vec())
            .with_timeout(5000)
            .with_priority(10)
            .with_metadata("traceparent", "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01")
            .with_transport_hint(TransportType::gRPC);

        assert_eq!(request.method, "fileSystem.readFile");
        assert_eq!(request.payload, b"{\"path\": \"/tmp/test.txt\"}");
        assert_eq!(request.timeout_ms(), Some(5000));
        assert_eq!(request.priority(), Some(10));
        assert_eq!(request.transport_hint, Some(TransportType::gRPC));
        assert!(request.metadata.contains_key("traceparent"));
    }

    #[test]
    fn test_unified_request_validation() {
        let mut request = UnifiedRequest::new("valid.method");
        assert!(request.validate().is_ok());

        request.method = String::new();
        assert!(request.validate().is_err());

        request.method = "valid.method".to_string();
        request.correlation_id = Some("".to_string());
        assert!(request.validate().is_err());
    }
}
