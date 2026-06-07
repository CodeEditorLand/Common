//! # Circuit Breaker Pattern
//!
//! Circuit breaker implementation for transport fault tolerance.
//! `CircuitBreakerState` lives in [`super::TransportStrategy`].

use std::time::Duration;

/// Configuration for the circuit breaker.
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfiguration {

	/// Number of consecutive failures before the circuit opens.
	pub FailureThreshold:u32,

	/// Duration to wait before transitioning to half-open.
	pub ResetTimeout:Duration,

	/// Successful requests in half-open state required to close the circuit.
	pub SuccessThreshold:u32,
}

impl Default for CircuitBreakerConfiguration {

	fn default() -> Self { Self { FailureThreshold:5, ResetTimeout:Duration::from_secs(60), SuccessThreshold:2 } }
}

/// Circuit breaker that wraps a transport to add fault-tolerance.
///
/// Tracks consecutive failures and opens the circuit when the
/// `FailureThreshold` is exceeded, preventing cascading failures.
#[derive(Debug, Clone)]
pub struct CircuitBreaker {

	Configuration:CircuitBreakerConfiguration,

	FailureCount:u32,

	IsOpen:bool,
}

impl CircuitBreaker {

	/// Creates a new circuit breaker with the given configuration.
	pub fn New(Configuration:CircuitBreakerConfiguration) -> Self {
		Self { Configuration, FailureCount:0, IsOpen:false }
	}

	/// Returns `true` if the circuit allows requests through.
	pub fn IsClosed(&self) -> bool { !self.IsOpen }

	/// Records a successful request, resetting the failure counter.
	pub fn RecordSuccess(&mut self) {
		self.FailureCount = 0;

		self.IsOpen = false;
	}

	/// Records a failed request, potentially opening the circuit.
	pub fn RecordFailure(&mut self) {
		self.FailureCount += 1;

		if self.FailureCount >= self.Configuration.FailureThreshold {
			self.IsOpen = true;
		}
	}
}
