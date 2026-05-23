//! # TransportStrategy Trait
//!
//! Defines the core trait that all transport implementations must implement.
//! This trait provides a unified, transport-agnostic interface for sending
//! requests and notifications, with optional event streaming capabilities.
//!
//! All transports must be async and thread-safe (`Send + Sync`).

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::{
	Common::TransportType,
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
#[async_trait]
pub trait TransportStrategy: Send + Sync {
	/// Establishes a connection to the transport endpoint.
	async fn Connect(&mut self) -> Result<(), TransportError>;

	/// Closes the connection and releases any associated resources.
	async fn Disconnect(&mut self) -> Result<(), TransportError>;

	/// Sends a request and waits for a response.
	async fn SendRequest(&mut self, Request:UnifiedRequest) -> Result<UnifiedResponse, TransportError>;

	/// Sends a notification (fire-and-forget message).
	async fn SendNotification(&mut self, Notification:UnifiedRequest) -> Result<(), TransportError>;

	/// Creates a stream of events from the transport.
	fn StreamEvents(&self)
	-> std::result::Result<futures::stream::BoxStream<'static, UnifiedResponse>, TransportError>;

	/// Checks if the transport is currently connected.
	fn IsConnected(&self) -> bool;

	/// Returns the estimated round-trip latency in milliseconds.
	fn LatencyMilliseconds(&self) -> u64;

	/// Returns the type of transport (gRPC, IPC, WASM, etc.).
	fn TransportKind(&self) -> TransportType;

	/// Returns the transport's configuration.
	fn Configuration(&self) -> &TransportConfig;

	/// Checks if the transport supports bidirectional streaming.
	fn SupportsStreaming(&self) -> bool;

	/// Returns the transport's current capabilities and limits.
	fn Capabilities(&self) -> TransportCapabilities;

	/// Collects and returns current performance metrics.
	fn Metrics(&self) -> TransportMetrics;
}

/// Transport capabilities and limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransportCapabilities {
	/// Maximum size of a single message in bytes.
	pub MaximumMessageSize:usize,

	/// Whether the transport supports request-response pattern.
	pub SupportsRequestResponse:bool,

	/// Whether the transport supports server-side streaming.
	pub SupportsServerStreaming:bool,

	/// Whether the transport supports client-side streaming.
	pub SupportsClientStreaming:bool,

	/// Whether the transport supports bidirectional streaming.
	pub SupportsBidirectionalStreaming:bool,

	/// Whether the transport supports broadcast/notifications.
	pub SupportsNotifications:bool,

	/// Estimated maximum concurrent requests/connections.
	pub MaximumConcurrent:usize,

	/// Whether the transport requires network connectivity.
	pub RequiresNetwork:bool,

	/// Whether the transport supports encryption/TLS.
	pub SupportsEncryption:bool,

	/// Whether the transport supports compression.
	pub SupportsCompression:bool,
}

impl Default for TransportCapabilities {
	fn default() -> Self {
		Self {
			MaximumMessageSize:1024 * 1024, // 1MB

			SupportsRequestResponse:true,

			SupportsServerStreaming:false,

			SupportsClientStreaming:false,

			SupportsBidirectionalStreaming:false,

			SupportsNotifications:true,

			MaximumConcurrent:100,

			RequiresNetwork:false,

			SupportsEncryption:false,

			SupportsCompression:false,
		}
	}
}

/// Transport performance metrics.
#[derive(Debug, Clone, Default)]
pub struct TransportMetrics {
	/// Total number of requests sent (including retries).
	pub RequestsTotal:u64,

	/// Total number of successful requests (2xx/OK responses).
	pub RequestsSuccessful:u64,

	/// Total number of failed requests (excludes timeouts/retries).
	pub RequestsFailed:u64,

	/// Total number of notifications sent.
	pub NotificationsSent:u64,

	/// Total number of connections established (includes reconnections).
	pub ConnectionsEstablished:u64,

	/// Total number of connection failures.
	pub ConnectionFailures:u64,

	/// Total bytes sent (compressed size if compression enabled).
	pub BytesSent:u64,

	/// Total bytes received (compressed size if compression enabled).
	pub BytesReceived:u64,

	/// Counter for circuit breaker state changes.
	pub CircuitBreakerState:u32,

	/// Histogram of request latencies in milliseconds (p50, p95, p99).
	/// Stored as (count, sum, sum of squares) for online variance calculation.
	pub LatencyMillisecondsHistogram:Option<(u64, f64, f64)>,

	/// Current active connections (gauge).
	pub ActiveConnections:u32,

	/// Current pending requests (gauge).
	pub PendingRequests:u32,
}

impl TransportMetrics {
	/// Creates a new, empty metrics container.
	pub fn New() -> Self { Self::default() }

	/// Resets all cumulative metrics to zero.
	pub fn Reset(&mut self) { *self = Self::New(); }

	/// Computes the success rate as a percentage (0-100).
	pub fn SuccessRate(&self) -> Option<f64> {
		let Total = self.RequestsTotal;

		if Total == 0 {
			None
		} else {
			Some((self.RequestsSuccessful as f64 / Total as f64) * 100.0)
		}
	}

	/// Computes the average request latency in milliseconds.
	pub fn AverageLatency(&self) -> Option<f64> {
		let (Count, Sum, _) = self.LatencyMillisecondsHistogram?;

		if Count == 0 { None } else { Some(Sum / Count as f64) }
	}

	/// Computes the 95th percentile latency from the histogram.
	pub fn LatencyPercentile95(&self) -> Option<f64> {
		let (Count, Mean, SumSquared) = self.LatencyMillisecondsHistogram?;

		if Count < 20 {
			return None;
		}

		let Variance = (SumSquared / Count as f64) - (Mean * Mean);

		let StandardDeviation = Variance.sqrt();

		Some(Mean + 1.645 * StandardDeviation)
	}

	/// Records a request latency sample.
	pub fn RecordLatency(&mut self, LatencyMilliseconds:f64) {
		let (Count, Sum, SumSquared) = self.LatencyMillisecondsHistogram.get_or_insert((0, 0.0, 0.0));

		*Count += 1;
		*Sum += LatencyMilliseconds;
		*SumSquared += LatencyMilliseconds * LatencyMilliseconds;
	}

	/// Increments the RequestsTotal and RequestsSuccessful counters.
	pub fn IncrementRequestSuccess(&mut self) {
		self.RequestsTotal += 1;

		self.RequestsSuccessful += 1;
	}

	/// Increments the RequestsTotal and RequestsFailed counters.
	pub fn IncrementRequestFailure(&mut self) {
		self.RequestsTotal += 1;

		self.RequestsFailed += 1;
	}

	/// Updates the circuit breaker state.
	pub fn SetCircuitBreakerState(&mut self, State:CircuitBreakerState) {
		let StateCode = match State {
			CircuitBreakerState::Closed => 1,

			CircuitBreakerState::Open => 0,

			CircuitBreakerState::HalfOpen => 2,
		};

		let OldState = self.CircuitBreakerState;

		self.CircuitBreakerState = (OldState & 0xFFFF_0000) | StateCode as u32;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
	pub fn IsRetryable(&self) -> bool {
		matches!(
			self,
			TransportErrorCode::ConnectionFailed
				| TransportErrorCode::Timeout
				| TransportErrorCode::RateLimited
				| TransportErrorCode::RemoteError
		)
	}

	/// Returns the recommended retry delay in milliseconds for this error.
	pub fn RecommendedRetryDelayMilliseconds(&self) -> u64 {
		match self {
			TransportErrorCode::ConnectionFailed => 1000,

			TransportErrorCode::Timeout => 500,

			TransportErrorCode::RateLimited => 2000,

			TransportErrorCode::RemoteError => 300,

			_ => 0,
		}
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn TestRetryableErrorCodes() {
		assert!(TransportErrorCode::ConnectionFailed.IsRetryable());

		assert!(TransportErrorCode::Timeout.IsRetryable());

		assert!(TransportErrorCode::RateLimited.IsRetryable());

		assert!(!TransportErrorCode::InvalidRequest.IsRetryable());

		assert!(!TransportErrorCode::NotFound.IsRetryable());
	}

	#[test]
	fn TestErrorRecommendedDelays() {
		assert_eq!(TransportErrorCode::ConnectionFailed.RecommendedRetryDelayMilliseconds(), 1000);

		assert_eq!(TransportErrorCode::Timeout.RecommendedRetryDelayMilliseconds(), 500);

		assert_eq!(TransportErrorCode::RateLimited.RecommendedRetryDelayMilliseconds(), 2000);

		assert_eq!(TransportErrorCode::InvalidRequest.RecommendedRetryDelayMilliseconds(), 0);
	}
}
