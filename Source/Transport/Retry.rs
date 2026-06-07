//! # Retry Strategies
//!
//! Retry configuration and strategies for transport operations.

use std::time::Duration;

/// Configuration for retry behaviour.
#[derive(Debug, Clone)]
pub struct RetryConfiguration {

	/// Maximum number of retry attempts.
	pub MaximumAttempts:u32,

	/// Base delay for exponential backoff.
	pub BaseDelay:Duration,

	/// Maximum delay cap.
	pub MaximumDelay:Duration,

	/// Whether to add jitter to retry delays.
	pub JitterEnabled:bool,
}

impl Default for RetryConfiguration {

	fn default() -> Self {
		Self {
			MaximumAttempts:3,

			BaseDelay:Duration::from_millis(100),

			MaximumDelay:Duration::from_secs(10),

			JitterEnabled:true,
		}
	}
}

/// Retry strategy selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryStrategy {

	/// No retries.
	None,

	/// Fixed delay between retries.
	Fixed,

	/// Exponential backoff.
	Exponential,

	/// Exponential backoff with jitter.
	ExponentialJitter,
}
