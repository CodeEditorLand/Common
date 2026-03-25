//! # TransportError Enum
//!
//! Defines the unified error type for all transport operations.
//!
//! The transport layer uses a structured error model that provides:
//!
//! - Transport-agnostic error codes
//! - Contextual information (method, correlation ID, etc.)
//! - Error classification (retryable vs. non-retryable)
//! - Diagnostic information for debugging

use std::fmt;

use super::TransportErrorCode::TransportErrorCode;

/// Unified transport error.
///
/// This error type captures all failure modes that can occur during
/// transport operations. It provides structured data for diagnostics,
/// metrics, and error handling logic (retries, circuit breaker, etc.).
///
/// # Error Classification
///
/// Errors are classified by `code` and can be tested for retryability:
///
/// - **Retryable**: Connection failures, timeouts, rate limits, transient remote errors
/// - **Non-retryable**: Invalid arguments, not found, unauthorized, configuration errors
///
/// # Context
///
/// Each error can carry rich context including:
///
/// - `method`: The RPC method being invoked (if applicable)
/// - `correlation_id`: The request ID for tracing
/// - `transport_type`: Which transport failed
/// - `retry_attempt`: How many retries preceded this failure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportError {
    /// Error code indicating the type of failure.
    pub code: TransportErrorCode,

    /// Human-readable error message.
    pub message: String,

    /// Optional underlying/boxed error (`Box<dyn std::error::Error>`).
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,

    /// The transport type that generated this error.
    pub transport_type: String,

    /// The method being invoked when the error occurred (if applicable).
    pub method: Option<String>,

    /// The correlation/request ID for tracing.
    pub correlation_id: Option<String>,

    /// Number of retry attempts before this failure (for cumulative failures).
    pub retry_attempt: u32,

    /// Additional error context as key-value pairs.
    pub context: std::collections::HashMap<String, String>,
}

impl TransportError {
    /// Creates a new `TransportError` with the given code and message.
    ///
    /// # Parameters
    ///
    /// * `code`: The error code from `TransportErrorCode`
    /// * `message`: Human-readable error description
    ///
    /// # Returns
    ///
    /// A new `TransportError` with default values for optional fields.
    pub fn new(code: TransportErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            source: None,
            transport_type: String::new(),
            method: None,
            correlation_id: None,
            retry_attempt: 0,
            context: std::collections::HashMap::new(),
        }
    }

    /// Sets the transport type on this error.
    ///
    /// This is typically set by the transport implementation itself.
    pub fn with_transport_type(mut self, transport_type: &str) -> Self {
        self.transport_type = transport_type.to_string();
        self
    }

    /// Sets the method name on this error.
    ///
    /// Useful for indicating which RPC method failed.
    pub fn with_method(mut self, method: &str) -> Self {
        self.method = Some(method.to_string());
        self
    }

    /// Sets the correlation/request ID on this error.
    ///
    /// This enables request tracing through logs and metrics.
    pub fn with_correlation_id(mut self, correlation_id: &str) -> Self {
        self.correlation_id = Some(correlation_id.to_string());
        self
    }

    /// Sets the retry attempt count.
    ///
    /// Indicates how many retries preceded this error.
    pub fn with_retry_attempt(mut self, retry_attempt: u32) -> Self {
        self.retry_attempt = retry_attempt;
        self
    }

    /// Adds a context key-value pair to this error.
    ///
    /// Context provides additional diagnostic information without
    /// cluttering the main error message.
    ///
    /// # Parameters
    ///
    /// * `key`: The context key (should be lowercase, alphanumeric)
    /// * `value`: The context value
    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }

    /// Sets the underlying source error.
    ///
    /// This allows chaining of errors while preserving the original
    /// error type for programmatic handling.
    pub fn with_source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    /// Returns `true` if this error is retryable.
    ///
    /// Retryable errors are typically transient and may succeed if
    /// attempted again after a delay. Non-retryable errors should not
    /// be retried.
    pub fn is_retryable(&self) -> bool {
        self.code.is_retryable()
    }

    /// Returns the recommended retry delay in milliseconds.
    ///
    /// This is based on the error code and can be used by retry logic
    /// to determine how long to wait before the next attempt.
    pub fn retry_delay_ms(&self) -> u64 {
        self.code.recommended_retry_delay_ms()
    }

    /// Returns the full error message with all context included.
    ///
    /// This format is suitable for logging and user-facing display.
    pub fn full_message(&self) -> String {
        let mut msg = self.message.clone();

        if let Some(method) = &self.method {
            msg.push_str(&format!(" (method: {})", method));
        }

        if let Some(correlation_id) = &self.correlation_id {
            msg.push_str(&format!(" (correlation_id: {})", correlation_id));
        }

        if !self.transport_type.is_empty() {
            msg.push_str(&format!(" (transport: {})", self.transport_type));
        }

        if self.retry_attempt > 0 {
            msg.push_str(&format!(" (retry: {})", self.retry_attempt));
        }

        if !self.context.is_empty() {
            let context_str = self
                .context
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(", ");
            msg.push_str(&format!(" (context: {{{}}})", context_str));
        }

        if let Some(source) = &self.source {
            msg.push_str(&format!(" (cause: {})", source));
        }

        msg
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_message())
    }
}

impl std::error::Error for TransportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_deref()
    }
}

/// Convenience constructors for common transport errors.
impl TransportError {
    /// Connection error: failed to connect or lost connection.
    pub fn connection(message: impl Into<String>) -> Self {
        Self::new(TransportErrorCode::ConnectionFailed, message)
            .with_transport_type("unknown")
    }

    /// Timeout error: operation exceeded deadline.
    pub fn timeout(message: impl Into<String>) -> Self {
        Self::new(TransportErrorCode::Timeout, message)
            .with_transport_type("unknown")
    }

    /// Invalid request error: bad parameters or format.
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self::new(TransportErrorCode::InvalidRequest, message)
    }

    /// Not supported error: feature not implemented by this transport.
    pub fn not_supported(message: impl Into<String>) -> Self {
        Self::new(TransportErrorCode::NotSupported, message)
    }

    /// Remote error: the remote endpoint returned an error.
    pub fn remote(message: impl Into<String>) -> Self {
        Self::new(TransportErrorCode::RemoteError, message)
    }

    /// Internal error: something went wrong inside the transport.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(TransportErrorCode::InternalError, message)
    }

    /// Circuit breaker open error: request rejected due to circuit breaker.
    pub fn circuit_breaker_open() -> Self {
        Self::new(TransportErrorCode::CircuitBreakerOpen, "Circuit breaker is open")
            .with_transport_type("unknown")
    }

    /// Rate limited error: too many requests.
    pub fn rate_limited(retry_after_ms: u64) -> Self {
        let mut error = Self::new(TransportErrorCode::RateLimited, "Rate limit exceeded")
            .with_context("retry_after_ms", &retry_after_ms.to_string());
        // Add retry-after header suggestion
        error.context.insert("retry_after".to_string(), format!("{}ms", retry_after_ms));
        error
    }

    /// Message too large error.
    pub fn message_too_large(size: usize, max_size: usize) -> Self {
        Self::new(
            TransportErrorCode::MessageTooLarge,
            format!("Message size {} exceeds maximum {}", size, max_size),
        )
        .with_context("size", &size.to_string())
        .with_context("max_size", &max_size.to_string())
    }

    /// Serialization error.
    pub fn serialization(message: impl Into<String>) -> Self {
        Self::new(TransportErrorCode::SerializationError, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_error_construction() {
        let error = TransportError::connection("Connection refused");
        assert_eq!(error.code, TransportErrorCode::ConnectionFailed);
        assert!(error.message.contains("Connection refused"));
    }

    #[test]
    fn test_error_context() {
        let error = TransportError::new(TransportErrorCode::Timeout, "Request timed out")
            .with_method("ping")
            .with_correlation_id("12345")
            .with_context("endpoint", "localhost:50051");

        assert_eq!(error.method, Some("ping".to_string()));
        assert_eq!(error.correlation_id, Some("12345".to_string()));
        assert_eq!(error.context.get("endpoint"), Some(&"localhost:50051".to_string()));
    }

    #[test]
    fn test_error_is_retryable() {
        let conn_error = TransportError::connection("Connection failed");
        assert!(conn_error.is_retryable());

        let invalid_error = TransportError::invalid_request("Bad params");
        assert!(!invalid_error.is_retryable());
    }

    #[test]
    fn test_error_full_message() {
        let error = TransportError::timeout("Operation timed out")
            .with_method("get_file")
            .with_correlation_id("abc-123")
            .with_transport_type("grpc");

        let full_msg = error.full_message();
        assert!(full_msg.contains("Operation timed out"));
        assert!(full_msg.contains("method: get_file"));
        assert!(full_msg.contains("correlation_id: abc-123"));
        assert!(full_msg.contains("transport: grpc"));
    }
}
