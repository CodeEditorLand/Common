#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
//! # Transport Common Types
//!
//! Shared types and utilities used across all transport implementations.

use std::{
	fmt,
	time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

/// Correlation ID type.
///
/// Used to uniquely identify requests and match responses to requests.
pub type CorrelationId = String;

/// Timestamp type.
///
/// Represents time in microseconds since Unix epoch (1970-01-01 00:00:00 UTC).
pub type Timestamp = u64;

/// Generator trait for correlation IDs.
pub trait CorrelationIdGenerator {
	/// Generates a new unique correlation ID.
	fn Generate() -> CorrelationId;
}

/// Default correlation ID generator using UUID v4.
pub struct UuidCorrelationIdGenerator;

impl CorrelationIdGenerator for UuidCorrelationIdGenerator {
	fn Generate() -> CorrelationId { uuid::Uuid::new_v4().to_string() }
}

/// Generator trait for timestamps.
pub trait TimestampGenerator {
	/// Returns the current timestamp in microseconds since Unix epoch.
	fn Now() -> Timestamp;
}

/// Default timestamp generator using system clock.
pub struct SystemTimestampGenerator;

impl TimestampGenerator for SystemTimestampGenerator {
	fn Now() -> Timestamp {
		SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map(|Duration| Duration.as_micros() as Timestamp)
			.unwrap_or(0)
	}
}

/// Transport type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportType {
	/// gRPC over HTTP/2
	Grpc,

	/// Native inter-process communication (Unix sockets, named pipes)
	Ipc,

	/// WebAssembly/WebWorker transport (browser)
	Wasm,

	/// Unknown or unspecified transport
	Unknown,
}

impl TransportType {
	/// Returns the string representation of this transport type.
	pub fn AsString(&self) -> &'static str {
		match self {
			Self::Grpc => "grpc",

			Self::Ipc => "ipc",

			Self::Wasm => "wasm",

			Self::Unknown => "unknown",
		}
	}
}

impl fmt::Display for TransportType {
	fn fmt(&self, Formatter:&mut fmt::Formatter<'_>) -> fmt::Result { Formatter.write_str(self.AsString()) }
}

impl std::str::FromStr for TransportType {
	type Err = anyhow::Error;

	fn from_str(Input:&str) -> Result<Self, Self::Err> {
		match Input.to_lowercase().as_str() {
			"grpc" => Ok(Self::Grpc),

			"ipc" => Ok(Self::Ipc),

			"wasm" => Ok(Self::Wasm),

			"unknown" => Ok(Self::Unknown),

			_ => Err(anyhow::anyhow!("Unknown transport type: {}", Input)),
		}
	}
}

/// Transport type detector.
///
/// Provides runtime detection of the appropriate transport based on
/// environment and capabilities.
pub trait TransportTypeDetector: Send + Sync {
	/// Detects the best available transport for the current environment.
	fn DetectBestTransport(&self) -> TransportType;

	/// Checks if a specific transport is available in the current environment.
	fn IsTransportAvailable(&self, TransportKind:TransportType) -> bool;

	/// Lists all available transports in the current environment.
	fn ListAvailableTransports(&self) -> Vec<TransportType>;
}

/// Default transport detector using environment detection.
pub struct DefaultTransportTypeDetector;

impl DefaultTransportTypeDetector {
	/// Lists all available transports (static convenience method).
	pub fn list_available_transports() -> Vec<TransportType> {
		let Instance = DefaultTransportTypeDetector;

		Instance.ListAvailableTransports()
	}
}

impl TransportTypeDetector for DefaultTransportTypeDetector {
	fn DetectBestTransport(&self) -> TransportType {
		#[cfg(target_arch = "wasm32")]
		{
			TransportType::Wasm
		}

		#[cfg(not(target_arch = "wasm32"))]
		{
			TransportType::Grpc
		}
	}

	fn IsTransportAvailable(&self, TransportKind:TransportType) -> bool {
		match TransportKind {
			TransportType::Grpc => true,

			TransportType::Ipc => {
				#[cfg(any(unix, windows))]
				{
					true
				}

				#[cfg(not(any(unix, windows)))]
				{
					false
				}
			},

			TransportType::Wasm => {
				#[cfg(target_arch = "wasm32")]
				{
					true
				}

				#[cfg(not(target_arch = "wasm32"))]
				{
					false
				}
			},

			TransportType::Unknown => false,
		}
	}

	fn ListAvailableTransports(&self) -> Vec<TransportType> {
		let mut Available = Vec::new();

		if self.IsTransportAvailable(TransportType::Grpc) {
			Available.push(TransportType::Grpc);
		}

		if self.IsTransportAvailable(TransportType::Ipc) {
			Available.push(TransportType::Ipc);
		}

		if self.IsTransportAvailable(TransportType::Wasm) {
			Available.push(TransportType::Wasm);
		}

		Available
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn TestTransportTypeAsString() {
		assert_eq!(TransportType::Grpc.AsString(), "grpc");

		assert_eq!(TransportType::Ipc.AsString(), "ipc");

		assert_eq!(TransportType::Wasm.AsString(), "wasm");

		assert_eq!(TransportType::Unknown.AsString(), "unknown");
	}

	#[test]
	fn TestTransportTypeFromString() {
		assert_eq!("grpc".parse::<TransportType>().unwrap(), TransportType::Grpc);

		assert_eq!("ipc".parse::<TransportType>().unwrap(), TransportType::Ipc);

		assert_eq!("wasm".parse::<TransportType>().unwrap(), TransportType::Wasm);

		assert!("invalid".parse::<TransportType>().is_err());
	}

	#[test]
	fn TestDefaultDetector() {
		let Available = DefaultTransportTypeDetector::list_available_transports();

		assert!(Available.contains(&TransportType::Grpc));
	}
}
