#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
//! # TransportConfig
//!
//! Configuration structures for transport implementations.

use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};

use super::Common::TransportType;

/// Global transport configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportConfig {
	/// Default timeout for requests that don't specify one.
	pub DefaultTimeout:Duration,

	/// Maximum number of retry attempts for retryable errors.
	pub MaximumRetries:u32,

	/// Base retry delay for exponential backoff.
	pub RetryBaseDelay:Duration,

	/// Maximum retry delay cap (prevents extremely long backoffs).
	pub RetryMaximumDelay:Duration,

	/// Whether retry with jitter is enabled (recommended for distributed
	/// systems).
	pub RetryJitterEnabled:bool,

	/// Circuit breaker failure threshold (number of consecutive failures before
	/// opening).
	pub CircuitBreakerFailureThreshold:u32,

	/// Circuit breaker reset timeout (how long to wait before half-open).
	pub CircuitBreakerResetTimeout:Duration,

	/// Whether health checks are enabled.
	pub HealthChecksEnabled:bool,

	/// Health check interval (how often to perform health checks).
	pub HealthCheckInterval:Duration,

	/// Metrics collection enabled flag.
	pub MetricsEnabled:bool,

	/// Transport-specific configuration overrides.
	#[serde(skip_serializing_if = "HashMap::is_empty")]
	pub TransportConfigurations:HashMap<TransportType, serde_json::Value>,

	/// Allowed transport types for auto-selection.
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub AllowedTransports:Vec<TransportType>,

	/// Forbidden transport types (never used even if available).
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub ForbiddenTransports:Vec<TransportType>,
}

impl Default for TransportConfig {
	fn default() -> Self {
		Self {
			DefaultTimeout:Duration::from_secs(30),
			MaximumRetries:3,
			RetryBaseDelay:Duration::from_millis(100),
			RetryMaximumDelay:Duration::from_secs(10),
			RetryJitterEnabled:true,
			CircuitBreakerFailureThreshold:5,
			CircuitBreakerResetTimeout:Duration::from_secs(60),
			HealthChecksEnabled:true,
			HealthCheckInterval:Duration::from_secs(30),
			MetricsEnabled:true,
			TransportConfigurations:HashMap::new(),
			AllowedTransports:Vec::new(),
			ForbiddenTransports:Vec::new(),
		}
	}
}

impl TransportConfig {
	/// Creates a new `TransportConfig` with default values.
	pub fn New() -> Self { Self::default() }

	/// Sets the default request timeout.
	pub fn WithDefaultTimeout(mut self, Timeout:Duration) -> Self {
		self.DefaultTimeout = Timeout;
		self
	}

	/// Sets the maximum number of retry attempts.
	pub fn WithMaximumRetries(mut self, MaximumRetries:u32) -> Self {
		self.MaximumRetries = MaximumRetries;
		self
	}

	/// Sets the base retry delay for exponential backoff.
	pub fn WithRetryBaseDelay(mut self, Delay:Duration) -> Self {
		self.RetryBaseDelay = Delay;
		self
	}

	/// Sets the maximum retry delay cap.
	pub fn WithRetryMaximumDelay(mut self, Delay:Duration) -> Self {
		self.RetryMaximumDelay = Delay;
		self
	}

	/// Enables or disables retry jitter.
	pub fn WithRetryJitter(mut self, Enabled:bool) -> Self {
		self.RetryJitterEnabled = Enabled;
		self
	}

	/// Sets the circuit breaker failure threshold.
	pub fn WithCircuitBreakerThreshold(mut self, Threshold:u32) -> Self {
		self.CircuitBreakerFailureThreshold = Threshold;
		self
	}

	/// Sets the circuit breaker reset timeout.
	pub fn WithCircuitBreakerResetTimeout(mut self, Timeout:Duration) -> Self {
		self.CircuitBreakerResetTimeout = Timeout;
		self
	}

	/// Enables or disables health checks.
	pub fn WithHealthChecksEnabled(mut self, Enabled:bool) -> Self {
		self.HealthChecksEnabled = Enabled;
		self
	}

	/// Sets the health check interval.
	pub fn WithHealthCheckInterval(mut self, Interval:Duration) -> Self {
		self.HealthCheckInterval = Interval;
		self
	}

	/// Enables or disables metrics collection.
	pub fn WithMetricsEnabled(mut self, Enabled:bool) -> Self {
		self.MetricsEnabled = Enabled;
		self
	}

	/// Adds a transport-specific configuration override.
	pub fn WithTransportConfiguration(mut self, TransportKind:TransportType, Configuration:serde_json::Value) -> Self {
		self.TransportConfigurations.insert(TransportKind, Configuration);
		self
	}

	/// Gets the transport-specific configuration for the given type, if any.
	pub fn GetTransportConfiguration(&self, TransportKind:TransportType) -> Option<&serde_json::Value> {
		self.TransportConfigurations.get(&TransportKind)
	}

	/// Sets the allowed transport types for auto-selection.
	pub fn WithAllowedTransports(mut self, Transports:Vec<TransportType>) -> Self {
		self.AllowedTransports = Transports;
		self
	}

	/// Adds a forbidden transport type.
	pub fn AddForbiddenTransport(mut self, TransportKind:TransportType) -> Self {
		self.ForbiddenTransports.push(TransportKind);
		self
	}

	/// Alias for `AddForbiddenTransport`.
	pub fn WithForbiddenTransport(self, TransportKind:TransportType) -> Self {
		self.AddForbiddenTransport(TransportKind)
	}

	/// Checks if a transport type is allowed by this configuration.
	pub fn IsAllowed(&self, TransportKind:TransportType) -> bool {
		if self.ForbiddenTransports.contains(&TransportKind) {
			return false;
		}
		if self.AllowedTransports.is_empty() {
			true
		} else {
			self.AllowedTransports.contains(&TransportKind)
		}
	}

	/// Gets the effective timeout for a request, considering request-specific
	/// overrides.
	pub fn EffectiveTimeout(&self, RequestTimeoutMilliseconds:Option<u64>) -> Duration {
		RequestTimeoutMilliseconds
			.map(Duration::from_millis)
			.unwrap_or(self.DefaultTimeout)
	}

	/// Gets the effective retry delay for a given attempt number, considering
	/// jitter.
	pub fn EffectiveRetryDelay(&self, Attempt:u32) -> Duration {
		let Multiplier = 1u32.checked_shl(Attempt.min(30)).unwrap_or(u32::MAX);
		let mut Delay = self.RetryBaseDelay.checked_mul(Multiplier).unwrap_or(self.RetryMaximumDelay);

		if Delay > self.RetryMaximumDelay {
			Delay = self.RetryMaximumDelay;
		}

		if self.RetryJitterEnabled {
			let Nanoseconds = std::time::SystemTime::now()
				.duration_since(std::time::UNIX_EPOCH)
				.map(|Duration| Duration.subsec_nanos())
				.unwrap_or(0);
			let JitterFraction = (Nanoseconds % 1000) as f64 / 500.0 - 1.0;
			let JitterAmount = Delay.as_millis() as f64 * 0.25;
			let AdjustedMilliseconds = (Delay.as_millis() as f64 + JitterFraction * JitterAmount).max(1.0) as u64;
			Delay = Duration::from_millis(AdjustedMilliseconds);
		}

		Delay
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn TestTransportConfigDefaults() {
		let Configuration = TransportConfig::default();
		assert_eq!(Configuration.DefaultTimeout, Duration::from_secs(30));
		assert_eq!(Configuration.MaximumRetries, 3);
		assert!(Configuration.HealthChecksEnabled);
		assert!(Configuration.MetricsEnabled);
	}

	#[test]
	fn TestTransportConfigBuilder() {
		let Configuration = TransportConfig::default()
			.WithDefaultTimeout(Duration::from_secs(60))
			.WithMaximumRetries(5)
			.WithRetryJitter(false);

		assert_eq!(Configuration.DefaultTimeout, Duration::from_secs(60));
		assert_eq!(Configuration.MaximumRetries, 5);
		assert!(!Configuration.RetryJitterEnabled);
	}

	#[test]
	fn TestIsAllowed() {
		let Configuration = TransportConfig::default();
		assert!(Configuration.IsAllowed(TransportType::Grpc));

		let Configuration = Configuration.WithForbiddenTransport(TransportType::Grpc);
		assert!(!Configuration.IsAllowed(TransportType::Grpc));
		assert!(Configuration.IsAllowed(TransportType::Ipc));

		let Configuration = Configuration.WithAllowedTransports(vec![TransportType::Ipc]);
		assert!(!Configuration.IsAllowed(TransportType::Grpc));
		assert!(Configuration.IsAllowed(TransportType::Ipc));
	}

	#[test]
	fn TestEffectiveTimeout() {
		let Configuration = TransportConfig::default().WithDefaultTimeout(Duration::from_secs(30));

		assert_eq!(Configuration.EffectiveTimeout(None), Duration::from_secs(30));
		assert_eq!(Configuration.EffectiveTimeout(Some(5000)), Duration::from_millis(5000));
	}

	#[test]
	fn TestEffectiveRetryDelay() {
		let Configuration = TransportConfig::default()
			.WithRetryBaseDelay(Duration::from_millis(100))
			.WithRetryMaximumDelay(Duration::from_secs(10))
			.WithRetryJitter(false);

		assert_eq!(Configuration.EffectiveRetryDelay(0), Duration::from_millis(100));
		assert_eq!(Configuration.EffectiveRetryDelay(1), Duration::from_millis(200));
		assert_eq!(Configuration.EffectiveRetryDelay(2), Duration::from_millis(400));
		assert_eq!(Configuration.EffectiveRetryDelay(10), Duration::from_secs(10));
	}
}
