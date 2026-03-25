//! # TransportConfig
//!
//! Configuration structures for transport implementations.
//!
//! This module defines the configuration types that control transport behavior.
//! All transports should respect these common configuration options while allowing
//! transport-specific extensions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::Common::TransportType;

/// Global transport configuration.
///
/// This structure defines the configuration that applies to all transports
/// or to transport selection logic. It's used when creating transports and
/// when the transport registry needs to auto-select appropriate transports.
///
/// # Configuration Hierarchy
///
/// 1. Global defaults (this struct)
/// 2. Transport-specific overrides (via `transport_configs` map)
/// 3. Per-request overrides (via `UnifiedRequest.metadata`)
///
/// # Example
///
/// ```rust,ignore
/// let config = TransportConfig::default()
///     .with_default_timeout(Duration::from_secs(30))
///     .with_max_retries(3)
///     .with_transport_config(TransportType::Grpc, grpc_config)
///     .with_transport_config(TransportType::Ipc, ipc_config);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportConfig {
    /// Default timeout for requests that don't specify one.
    pub default_timeout: Duration,

    /// Maximum number of retry attempts for retryable errors.
    pub max_retries: u32,

    /// Base retry delay for exponential backoff.
    pub retry_base_delay: Duration,

    /// Maximum retry delay cap (prevents extremely long backoffs).
    pub retry_max_delay: Duration,

    /// Whether retry with jitter is enabled (recommended for distributed systems).
    pub retry_jitter_enabled: bool,

    /// Circuit breaker failure threshold (number of consecutive failures before opening).
    pub circuit_breaker_failure_threshold: u32,

    /// Circuit breaker reset timeout (how long to wait before half-open).
    pub circuit_breaker_reset_timeout: Duration,

    /// Whether health checks are enabled.
    pub health_checks_enabled: bool,

    /// Health check interval (how often to perform health checks).
    pub health_check_interval: Duration,

    /// Metrics collection enabled flag.
    pub metrics_enabled: bool,

    /// Transport-specific configuration overrides.
    /// Each transport type can have its own configuration structure.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub transport_configs: HashMap<TransportType, serde_json::Value>,

    /// Allowed transport types for auto-selection.
    /// If empty, all configured transports are considered.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_transports: Vec<TransportType>,

    /// Forbidden transport types (never used even if available).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub forbidden_transports: Vec<TransportType>,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            default_timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_base_delay: Duration::from_millis(100),
            retry_max_delay: Duration::from_secs(10),
            retry_jitter_enabled: true,
            circuit_breaker_failure_threshold: 5,
            circuit_breaker_reset_timeout: Duration::from_secs(60),
            health_checks_enabled: true,
            health_check_interval: Duration::from_secs(30),
            metrics_enabled: true,
            transport_configs: HashMap::new(),
            allowed_transports: Vec::new(),
            forbidden_transports: Vec::new(),
        }
    }
}

impl TransportConfig {
    /// Creates a new `TransportConfig` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the default request timeout.
    pub fn with_default_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    /// Sets the maximum number of retry attempts.
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Sets the base retry delay for exponential backoff.
    pub fn with_retry_base_delay(mut self, delay: Duration) -> Self {
        self.retry_base_delay = delay;
        self
    }

    /// Sets the maximum retry delay cap.
    pub fn with_retry_max_delay(mut self, delay: Duration) -> Self {
        self.retry_max_delay = delay;
        self
    }

    /// Enables or disables retry jitter.
    pub fn with_retry_jitter(mut self, enabled: bool) -> Self {
        self.retry_jitter_enabled = enabled;
        self
    }

    /// Sets the circuit breaker failure threshold.
    pub fn with_circuit_breaker_threshold(mut self, threshold: u32) -> Self {
        self.circuit_breaker_failure_threshold = threshold;
        self
    }

    /// Sets the circuit breaker reset timeout.
    pub fn with_circuit_breaker_reset_timeout(mut self, timeout: Duration) -> Self {
        self.circuit_breaker_reset_timeout = timeout;
        self
    }

    /// Enables or disables health checks.
    pub fn with_health_checks_enabled(mut self, enabled: bool) -> Self {
        self.health_checks_enabled = enabled;
        self
    }

    /// Sets the health check interval.
    pub fn with_health_check_interval(mut self, interval: Duration) -> Self {
        self.health_check_interval = interval;
        self
    }

    /// Enables or disables metrics collection.
    pub fn with_metrics_enabled(mut self, enabled: bool) -> Self {
        self.metrics_enabled = enabled;
        self
    }

    /// Adds a transport-specific configuration override.
    ///
    /// # Parameters
    ///
    /// * `transport_type`: The transport to which this config applies
    /// * `config`: JSON-serializable configuration for that transport
    pub fn with_transport_config(
        mut self,
        transport_type: TransportType,
        config: serde_json::Value,
    ) -> Self {
        self.transport_configs.insert(transport_type, config);
        self
    }

    /// Gets the transport-specific configuration for the given type, if any.
    pub fn get_transport_config(&self, transport_type: TransportType) -> Option<&serde_json::Value> {
        self.transport_configs.get(&transport_type)
    }

    /// Sets the allowed transport types for auto-selection.
    ///
    /// Transports not in this list will be ignored during auto-selection.
    pub fn with_allowed_transports(mut self, transports: Vec<TransportType>) -> Self {
        self.allowed_transports = transports;
        self
    }

    /// Adds a forbidden transport type.
    ///
    /// Forbidden transports will never be selected, even if available.
    pub fn add_forbidden_transport(mut self, transport_type: TransportType) -> Self {
        self.forbidden_transports.push(transport_type);
        self
    }

    /// Checks if a transport type is allowed by this configuration.
    pub fn is_allowed(&self, transport_type: TransportType) -> bool {
        if self.forbidden_transports.contains(&transport_type) {
            return false;
        }

        if self.allowed_transports.is_empty() {
            true // All allowed if no whitelist
        } else {
            self.allowed_transports.contains(&transport_type)
        }
    }

    /// Gets the effective timeout for a request, considering request-specific overrides.
    ///
    /// # Parameters
    ///
    /// * `request_timeout_ms`: Optional timeout from request metadata
    ///
    /// # Returns
    ///
    /// The timeout to use, taking the most specific override.
    pub fn effective_timeout(&self, request_timeout_ms: Option<u64>) -> Duration {
        request_timeout_ms
            .map(Duration::from_millis)
            .unwrap_or(self.default_timeout)
    }

    /// Gets the effective retry delay for a given attempt number, considering jitter.
    ///
    /// # Parameters
    ///
    /// * `attempt`: The retry attempt number (0-based)
    ///
    /// # Returns
    ///
    /// The delay to wait before the next retry.
    pub fn effective_retry_delay(&self, attempt: u32) -> Duration {
        // Exponential backoff: base_delay * 2^attempt
        let mut delay = self
            .retry_base_delay
            .checked_mul(Duration::from_secs(1 << attempt))
            .unwrap_or(self.retry_max_delay);

        // Cap at max delay
        if delay > self.retry_max_delay {
            delay = self.retry_max_delay;
        }

        // Add jitter if enabled: ±25% random adjustment
        if self.retry_jitter_enabled {
            let jitter_percent = 0.25;
            let jitter_amount = delay.as_millis() as f64 * jitter_percent;
            let jitter = (rand::random::<f64>() * 2.0 - 1.0) * jitter_amount;
            let adjusted_ms = (delay.as_millis() as f64 + jitter).max(1.0) as u64;
            delay = Duration::from_millis(adjusted_ms);
        }

        delay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_config_defaults() {
        let config = TransportConfig::default();
        assert_eq!(config.default_timeout, Duration::from_secs(30));
        assert_eq!(config.max_retries, 3);
        assert!(config.health_checks_enabled);
        assert!(config.metrics_enabled);
    }

    #[test]
    fn test_transport_config_builder() {
        let config = TransportConfig::default()
            .with_default_timeout(Duration::from_secs(60))
            .with_max_retries(5)
            .with_retry_jitter(false);

        assert_eq!(config.default_timeout, Duration::from_secs(60));
        assert_eq!(config.max_retries, 5);
        assert!(!config.retry_jitter_enabled);
    }

    #[test]
    fn test_is_allowed() {
        let config = TransportConfig::default();
        assert!(config.is_allowed(TransportType::gRPC));

        let config = config.with_forbidden_transport(TransportType::gRPC);
        assert!(!config.is_allowed(TransportType::gRPC));
        assert!(config.is_allowed(TransportType::IPC));

        let config = config.with_allowed_transports(vec![TransportType::IPC]);
        assert!(!config.is_allowed(TransportType::gRPC));
        assert!(config.is_allowed(TransportType::IPC));
    }

    #[test]
    fn test_effective_timeout() {
        let config = TransportConfig::default()
            .with_default_timeout(Duration::from_secs(30));

        // No override
        assert_eq!(config.effective_timeout(None), Duration::from_secs(30));

        // With override
        assert_eq!(config.effective_timeout(Some(5000)), Duration::from_millis(5000));
    }

    #[test]
    fn test_effective_retry_delay() {
        let config = TransportConfig::default()
            .with_retry_base_delay(Duration::from_millis(100))
            .with_retry_max_delay(Duration::from_secs(10))
            .with_retry_jitter(false);

        // First attempt (0) -> base * 2^0 = 100ms
        assert_eq!(config.effective_retry_delay(0), Duration::from_millis(100));

        // Second attempt (1) -> base * 2^1 = 200ms
        assert_eq!(config.effective_retry_delay(1), Duration::from_millis(200));

        // Third attempt (2) -> base * 2^2 = 400ms
        assert_eq!(config.effective_retry_delay(2), Duration::from_millis(400));

        // Large attempt should cap at max delay
        assert_eq!(config.effective_retry_delay(10), Duration::from_secs(10));
    }
}
