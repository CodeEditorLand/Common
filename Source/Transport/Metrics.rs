#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
//! # Transport Metrics
//!
//! Metrics collection for transport operations.
//! Core metrics types live in [`super::TransportStrategy`].

/// Trait for collecting transport metrics.
///
/// Implementations record request outcomes and latency samples,
/// and produce snapshots for monitoring and diagnostics.
pub trait MetricsCollector: Send + Sync {
	/// Records a completed request.
	///
	/// * `Success` - whether the request succeeded
	/// * `LatencyMilliseconds` - round-trip latency in milliseconds
	fn RecordRequest(&self, Success: bool, LatencyMilliseconds: f64);
}
