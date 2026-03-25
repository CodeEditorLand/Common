//! # Transport Common Types
//!
//! Shared types and utilities used across all transport implementations.
//!
//! This module provides:
//!
//! - [`CorrelationId`] - UUID type for request correlation
//! - [`Timestamp`] - Microsecond timestamp type
//! - [`TransportType`] - Transport kind enumeration
//! - [`TransportTypeDetector`] - Runtime transport detection
//! - [`CorrelationIdGenerator`] - Trait for generating correlation IDs
//! - [`TimestampGenerator`] - Trait for getting current timestamps

use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Correlation ID type.
///
/// Used to uniquely identify requests and match responses to requests.
pub type CorrelationId = String;

/// Timestamp type.
///
/// Represents time in microseconds since Unix epoch (1970-01-01 00:00:00 UTC).
pub type Timestamp = u64;

/// Generator trait for correlation IDs.
///
/// This allows different ID generation strategies (UUID, sequential, etc.)
/// to be injected for testing or special requirements.
pub trait CorrelationIdGenerator {
    /// Generates a new unique correlation ID.
    fn generate() -> CorrelationId;
}

/// Default correlation ID generator using UUID v4.
pub struct UuidCorrelationIdGenerator;

impl CorrelationIdGenerator for UuidCorrelationIdGenerator {
    fn generate() -> CorrelationId {
        uuid::Uuid::new_v4().to_string()
    }
}

/// Generator trait for timestamps.
///
/// Allows time to be injected for testing (mock clocks) or obtained
/// from system clocks.
pub trait TimestampGenerator {
    /// Returns the current timestamp in microseconds since Unix epoch.
    fn now() -> Timestamp;
}

/// Default timestamp generator using system clock.
pub struct SystemTimestampGenerator;

impl TimestampGenerator for SystemTimestampGenerator {
    fn now() -> Timestamp {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_micros() as Timestamp)
            .unwrap_or(0)
    }
}

/// Transport type enumeration.
///
/// Indicates which transport mechanism is being used or requested.
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
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Grpc => "grpc",
            Self::Ipc => "ipc",
            Self::Wasm => "wasm",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for TransportType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for TransportType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "grpc" => Ok(Self::Grpc),
            "ipc" => Ok(Self::Ipc),
            "wasm" => Ok(Self::Wasm),
            "unknown" => Ok(Self::Unknown),
            _ => Err(anyhow::anyhow!("Unknown transport type: {}", s)),
        }
    }
}

/// Transport type detector.
///
/// Provides runtime detection of the appropriate transport based on
/// environment and capabilities.
pub trait TransportTypeDetector {
    /// Detects the best available transport for the current environment.
    fn detect_best_transport() -> TransportType;

    /// Checks if a specific transport is available in the current environment.
    fn is_transport_available(transport_type: TransportType) -> bool;

    /// Lists all available transports in the current environment.
    fn list_available_transports() -> Vec<TransportType>;
}

/// Default transport detector using environment detection.
pub struct DefaultTransportTypeDetector;

impl TransportTypeDetector for DefaultTransportTypeDetector {
    fn detect_best_transport() -> TransportType {
        // Priority order based on environment:
        // - If in browser and WASM supported: WASM
        // - If same-process and IPC available: IPC
        // - Otherwise: gRPC

        #[cfg(target_arch = "wasm32")]
        {
            // In browser, prefer WASM
            TransportType::Wasm
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // On desktop/server, check if we're in same process
            // For now, default to gRPC (cross-process)
            TransportType::Grpc
        }
    }

    fn is_transport_available(transport_type: TransportType) -> bool {
        match transport_type {
            TransportType::Grpc => true, // gRPC always available
            TransportType::Ipc => {
                #[cfg(unix)]
                {
                    true // Unix sockets available
                }
                #[cfg(windows)]
                {
                    true // Named pipes available
                }
            }
            TransportType::Wasm => {
                #[cfg(target_arch = "wasm32")]
                {
                    true
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    false // WASM not available outside browser
                }
            }
            TransportType::Unknown => false,
        }
    }

    fn list_available_transports() -> Vec<TransportType> {
        let mut available = Vec::new();

        if Self::is_transport_available(TransportType::Grpc) {
            available.push(TransportType::Grpc);
        }
        if Self::is_transport_available(TransportType::Ipc) {
            available.push(TransportType::Ipc);
        }
        if Self::is_transport_available(TransportType::Wasm) {
            available.push(TransportType::Wasm);
        }

        available
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_type_as_str() {
        assert_eq!(TransportType::Grpc.as_str(), "grpc");
        assert_eq!(TransportType::Ipc.as_str(), "ipc");
        assert_eq!(TransportType::Wasm.as_str(), "wasm");
        assert_eq!(TransportType::Unknown.as_str(), "unknown");
    }

    #[test]
    fn test_transport_type_from_str() {
        assert_eq!("grpc".parse().unwrap(), TransportType::Grpc);
        assert_eq!("ipc".parse().unwrap(), TransportType::Ipc);
        assert_eq!("wasm".parse().unwrap(), TransportType::Wasm);
        assert!("invalid".parse::<TransportType>().is_err());
    }

    #[test]
    fn test_default_detector() {
        let available = DefaultTransportTypeDetector::list_available_transports();
        // At minimum gRPC should be available
        assert!(available.contains(&TransportType::Grpc));
    }
}
