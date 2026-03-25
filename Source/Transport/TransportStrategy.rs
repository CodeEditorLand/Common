//! # TransportStrategy Trait
//!
//! Defines the core trait that all transport implementations must implement.
//! This trait provides a unified, transport-agnostic interface for sending
//! requests and notifications, with optional event streaming capabilities.
//!
//! All transports must be async and thread-safe (`Send + Sync`).

use async_trait::async_trait;
use std::time::Duration;

use super::{
    Common::{
        CorrelationId, CorrelationIdGenerator, Timestamp, TimestampGenerator, TransportType,
        TransportTypeDetector,
    },
    TransportConfig::TransportConfig,
    TransportError::TransportError,
    UnifiedRequest::UnifiedRequest,
    UnifiedResponse::UnifiedResponse,
};

/// Core transport strategy trait.
///
/// This trait defines the essential operations that any transport mechanism
/// must provide. Components interact with transports through this trait,
/// allowing them to be transport-agnostic.
///
/// # Type Parameters
///
/// The trait does not use generic parameters to allow dynamic dispatch via
/// `Box<dyn TransportStrategy>`. All request/response types use the unified
/// protocol-agnostic `UnifiedRequest` and `UnifiedResponse` DTOs.
#[async_trait]
pub trait TransportStrategy: Send + Sync {
    /// Establishes a connection to the transport endpoint.
    ///
    /// This method should be idempotent - calling it multiple times on an
    /// already connected transport should return `Ok(())` without side effects.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if connection is established (or already connected)
    /// - `Err(TransportError)` if connection fails
    async fn connect(&mut self) -> Result<(), TransportError>;

    /// Closes the connection and releases any associated resources.
    ///
    /// This method should be safe to call multiple times and on an already
    /// disconnected transport.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if disconnection succeeds
    /// - `Err(TransportError)` if an error occurs during cleanup
    async fn disconnect(&mut self) -> Result<(), TransportError>;

    /// Sends a request and waits for a response.
    ///
    /// This is the primary request-response pattern used for RPC-style
    /// communication. The request must include a unique correlation ID for
    /// matching with the response (if not provided, the transport may
    /// generate one).
    ///
    /// # Parameters
    ///
    /// * `request`: The `UnifiedRequest` containing method, payload, and metadata
    ///
    /// # Returns
    ///
    /// - `Ok(UnifiedResponse)` containing the response payload and status
    /// - `Err(TransportError)` if the request fails or times out
    ///
    /// # Timeouts
    ///
    /// Transports should respect the timeout in `request.metadata.timeout_ms`
    /// if present. If no timeout is specified, a reasonable default should
    /// be used (typically 30 seconds).
    async fn send_request(&mut self, request: UnifiedRequest)
        -> Result<UnifiedResponse, TransportError>;

    /// Sends a notification (fire-and-forget message).
    ///
    /// Unlike `send_request`, this method does not wait for any response and
    /// returns immediately. Useful for events, logging, or best-effort
    /// operations where the sender doesn't care about the result.
    ///
    /// # Parameters
    ///
    /// * `notification`: The `UnifiedRequest` containing event data
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the notification was successfully sent (not delivered!)
    /// - `Err(TransportError)` if sending fails (e.g., disconnected, buffer overflow)
    async fn send_notification(&mut self, notification: UnifiedRequest)
        -> Result<(), TransportError>;

    /// Creates a stream of events from the transport.
    ///
    /// This method enables server-to-client streaming (push notifications,
    /// real-time updates, etc.). The returned stream yields events until
    /// the transport is disconnected or the stream is cancelled.
    ///
    /// # Returns
    ///
    /// - `Ok(BoxStream<'static, UnifiedResponse>)` if streaming is supported
    /// - `Err(TransportError)` if streaming is not supported by this transport
    ///
    /// # Streaming Support
    ///
    /// Not all transports support bidirectional streaming. Transports that
    /// don't support streaming should return `TransportError::NotSupported`.
    /// Callers should check `supports_streaming()` before calling this method.
    ///
    /// # Stream Lifetime
    ///
    /// The returned stream is `'static` and can be sent across threads. It
    /// will automatically terminate when the underlying transport disconnects.
    fn stream_events(
        &self,
    ) -> std::result::Result<futures::stream::BoxStream<'static, UnifiedResponse>, TransportError>;

    /// Checks if the transport is currently connected.
    ///
    /// # Returns
    ///
    /// - `true` if the transport is connected and ready to send/receive
    /// - `false` if disconnected or in the process of connecting
    fn is_connected(&self) -> bool;

    /// Returns the estimated round-trip latency in milliseconds.
    ///
    /// This is an operational metric that may be measured or estimated.
    /// It's used for diagnostics and transport selection.
    ///
    /// # Returns
    ///
    /// - Current latency estimate in milliseconds (ms)
    /// - `u64::MAX` if latency is unknown or cannot be measured
    fn latency_ms(&self) -> u64;

    /// Returns the type of transport (gRPC, IPC, WASM, etc.).
    ///
    /// This is used for diagnostics, metrics labeling, and transport selection.
    fn transport_type(&self) -> TransportType;

    /// Returns the transport's configuration.
    ///
    /// The configuration is immutable after creation, but callers may inspect
    /// it to understand transport capabilities and limits.
    fn config(&self) -> &TransportConfig;

    /// Checks if the transport supports bidirectional streaming.
    ///
    /// This is a quick capability check that doesn't require allocating a
    /// stream. Callers should use this to avoid the overhead of attempting
    /// to create a stream when it's not supported.
    ///
    /// # Returns
    ///
    /// - `true` if `stream_events()` is expected to succeed
    /// - `false` if streaming is not supported
    fn supports_streaming(&self) -> bool;

    /// Returns the transport's current capabilities and limits.
    ///
    /// This provides a snapshot of what the transport can do, including
    /// max message size, connection limits, supported features, etc.
    fn capabilities(&self) -> TransportCapabilities;

    /// Collects and returns current performance metrics.
    ///
    /// This method aggregates all metrics collected by the transport and
    /// returns them as a structured snapshot. Metrics are cumulative since
    /// the transport was created or last reset.
    ///
    /// # Returns
    ///
    /// A `TransportMetrics` struct containing counters, histograms, gauges, etc.
    fn metrics(&self) -> TransportMetrics;
}

/// Transport capabilities and limits.
///
/// This structure describes what a transport instance is capable of and
/// various operational limits. It's typically static for a given transport
/// implementation and configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransportCapabilities {
    /// Maximum size of a single message in bytes.
    pub max_message_size: usize,

    /// Whether the transport supports request-response pattern.
    pub supports_request_response: bool,

    /// Whether the transport supports server-side streaming.
    pub supports_server_streaming: bool,

    /// Whether the transport supports client-side streaming.
    pub supports_client_streaming: bool,

    /// Whether the transport supports bidirectional streaming.
    pub supports_bidirectional_streaming: bool,

    /// Whether the transport supports broadcast/notifications.
    pub supports_notifications: bool,

    /// Estimated maximum concurrent requests/connections.
    pub max_concurrent: usize,

    /// Whether the transport requires network connectivity.
    pub requires_network: bool,

    /// Whether the transport supports encryption/TLS.
    pub supports_encryption: bool,

    /// Whether the transport supports compression.
    pub supports_compression: bool,
}

impl Default for TransportCapabilities {
    fn default() -> Self {
        Self {
            max_message_size: 1024 * 1024, // 1MB
            supports_request_response: true,
            supports_server_streaming: false,
            supports_client_streaming: false,
            supports_bidirectional_streaming: false,
            supports_notifications: true,
            max_concurrent: 100,
            requires_network: false,
            supports_encryption: false,
            supports_compression: false,
        }
    }
}

/// Transport performance metrics.
///
/// This structure contains cumulative metrics collected by a transport
/// since it was created or last reset. These metrics are used for
/// monitoring, observability, and performance tuning.
#[derive(Debug, Clone, Default)]
pub struct TransportMetrics {
    /// Total number of requests sent (including retries).
    pub requests_total: u64,

    /// Total number of successful requests (2xx/OK responses).
    pub requests_successful: u64,

    /// Total number of failed requests (excludes timeouts/retries).
    pub requests_failed: u64,

    /// Total number of notifications sent.
    pub notifications_sent: u64,

    /// Total number of connections established (includes reconnections).
    pub connections_established: u64,

    /// Total number of connection failures.
    pub connection_failures: u64,

    /// Total bytes sent (compressed size if compression enabled).
    pub bytes_sent: u64,

    /// Total bytes received (compressed size if compression enabled).
    pub bytes_received: u64,

    /// Counter for circuit breaker state changes.
    /// Bit 0: 1=closed, 0=open; Bit 1: half-open flag; Bits 2-31: transition count
    pub circuit_breaker_state: u32,

    /// Histogram of request latencies in milliseconds (p50, p95, p99).
    /// Stored as (count, sum, sum of squares) for online variance calculation.
    pub latency_ms_histogram: Option<(u64, f64, f64)>,

    /// Current active connections (gauge).
    pub active_connections: u32,

    /// Current pending requests (gauge).
    pub pending_requests: u32,
}

impl TransportMetrics {
    /// Creates a new, empty metrics container.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets all cumulative metrics to zero.
    ///
    /// This is useful for measuring intervals or for periodic reporting
    /// without losing the transport instance.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Computes the success rate as a percentage (0-100).
    ///
    /// # Returns
    ///
    /// - `Some(f64)` with percentage (0-100) if there have been requests
    /// - `None` if no requests have been made yet
    pub fn success_rate(&self) -> Option<f64> {
        let total = self.requests_total;
        if total == 0 {
            None
        } else {
            Some((self.requests_successful as f64 / total as f64) * 100.0)
        }
    }

    /// Computes the average request latency in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Some(f64)` with average latency if there are datapoints
    /// - `None` if no latency data recorded
    pub fn average_latency(&self) -> Option<f64> {
        let (count, sum, _) = self.latency_ms_histogram?;
        if count == 0 {
            None
        } else {
            Some(sum / count as f64)
        }
    }

    /// Computes the 95th percentile latency from the histogram.
    ///
    /// This is an approximate calculation using the sum of squares.
    /// For precise percentiles, data should be exported to a proper
    /// metrics system (Prometheus, Grafana, etc.).
    ///
    /// # Returns
    ///
    /// - `Some(f64)` with p95 latency in ms
    /// - `None` if insufficient data
    pub fn latency_p95(&self) -> Option<f64> {
        let (count, mean, sum_sq) = self.latency_ms_histogram?;
        if count < 20 {
            // Need at least 20 samples for meaningful p95
            return None;
        }
        // Approximate: mean + 1.645 * stddev
        let variance = (sum_sq / count as f64) - (mean * mean);
        let stddev = variance.sqrt();
        Some(mean + 1.645 * stddev)
    }

    /// Records a request latency sample.
    ///
    /// This updates the histogram statistics with a new latency measurement.
    ///
    /// # Parameters
    ///
    /// * `latency_ms`: The observed latency in milliseconds
    pub fn record_latency(&mut self, latency_ms: f64) {
        let (count, sum, sum_sq) = self.latency_ms_histogram.get_or_insert((0, 0.0, 0.0));
        *count += 1;
        *sum += latency_ms;
        *sum_sq += latency_ms * latency_ms;
    }

    /// Increments the requests_total and requests_successful counters.
    ///
    /// This should be called when a request completes successfully.
    pub fn increment_request_success(&mut self) {
        self.requests_total += 1;
        self.requests_successful += 1;
    }

    /// Increments the requests_total and requests_failed counters.
    ///
    /// This should be called when a request fails with a non-retryable error.
    pub fn increment_request_failure(&mut self) {
        self.requests_total += 1;
        self.requests_failed += 1;
    }

    /// Updates the circuit breaker state.
    ///
    /// # Parameters
    ///
    /// * `state`: The new circuit breaker state (0=open, 1=closed, 2=half-open)
    pub fn set_circuit_breaker_state(&mut self, state: CircuitBreakerState) {
        let state_code = match state {
            CircuitBreakerState::Closed => 1,
            CircuitBreakerState::Open => 0,
            CircuitBreakerState::HalfOpen => 2,
        };
        // Increment transition count in upper 31 bits
        let old_state = self.circuit_breaker_state;
        self.circuit_breaker_state = (old_state & 0xFFFF_0000) | state_code as u32;
    }
}

/// Circuit breaker state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitBreakerState {
    /// Circuit is closed; requests flow normally.
    Closed,
    /// Circuit is open; requests are rejected immediately.
    Open,
    /// Circuit is half-open; limited requests are allowed to test recovery.
    HalfOpen,
}

/// Transport-specific error codes.
///
/// These codes are transport-agnostic and map to specific failure modes.
/// They are used for metrics, diagnostics, and error handling strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum TransportErrorCode {
    /// Connection to endpoint failed or was lost.
    ConnectionFailed = 100,
    /// Operation timed out.
    Timeout = 101,
    /// Target endpoint not found/service unavailable.
    NotFound = 102,
    /// Invalid request format or parameters.
    InvalidRequest = 103,
    /// Remote endpoint returned an application error.
    RemoteError = 104,
    /// Message too large for transport.
    MessageTooLarge = 105,
    /// Encryption/decryption failed.
    EncryptionError = 106,
    /// Serialization/deserialization failed.
    SerializationError = 107,
    /// Authentication/authorization failed.
    Unauthorized = 108,
    /// Rate limit exceeded.
    RateLimited = 109,
    /// Feature not supported by this transport.
    NotSupported = 110,
    /// Internal transport error (bug, corrupted state).
    InternalError = 111,
    /// Circuit breaker is open; request rejected.
    CircuitBreakerOpen = 112,
    /// Stream already in use or closed.
    StreamError = 113,
    /// Configuration error (invalid settings).
    ConfigurationError = 114,
}

impl TransportErrorCode {
    /// Returns `true` if this error code is retryable.
    ///
    /// Retryable errors are typically transient failures that may succeed
    /// if attempted again after a brief delay. Non-retryable errors should
    /// not be retried to avoid overwhelming the system.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            TransportErrorCode::ConnectionFailed
                | TransportErrorCode::Timeout
                | TransportErrorCode::RateLimited
                | TransportErrorCode::RemoteError // May be transient depending on code
        )
    }

    /// Returns the recommended retry delay in milliseconds for this error.
    ///
    /// The delay is based on the error type and may be adjusted by the
    /// retry strategy's backoff configuration.
    pub fn recommended_retry_delay_ms(&self) -> u64 {
        match self {
            TransportErrorCode::ConnectionFailed => 1000,
            TransportErrorCode::Timeout => 500,
            TransportErrorCode::RateLimited => 2000,
            TransportErrorCode::RemoteError => 300,
            _ => 0, // No retry
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retryable_error_codes() {
        assert!(TransportErrorCode::ConnectionFailed.is_retryable());
        assert!(TransportErrorCode::Timeout.is_retryable());
        assert!(TransportErrorCode::RateLimited.is_retryable());
        assert!(!TransportErrorCode::InvalidRequest.is_retryable());
        assert!(!TransportErrorCode::NotFound.is_retryable());
    }

    #[test]
    fn test_error_recommended_delays() {
        assert_eq!(
            TransportErrorCode::ConnectionFailed.recommended_retry_delay_ms(),
            1000
        );
        assert_eq!(TransportErrorCode::Timeout.recommended_retry_delay_ms(), 500);
        assert_eq!(
            TransportErrorCode::RateLimited.recommended_retry_delay_ms(),
            2000
        );
        assert_eq!(
            TransportErrorCode::InvalidRequest.recommended_retry_delay_ms(),
            0
        );
    }
}
