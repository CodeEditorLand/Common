//! # Transport DTOs
//!
//! Data Transfer Objects used by the transport layer.
//!
//! This module re-exports all DTOs defined in the transport system, providing
//! a single import location for all transport-related types.

// Core DTOs
pub mod unified_request;
pub mod unified_response;
pub mod transport_error;
pub mod correlation;

// Re-exports
pub use unified_request::UnifiedRequest;
pub use unified_response::{UnifiedResponse, ResponseError};
pub use transport_error::{TransportError, TransportErrorCode};
pub use correlation::{CorrelationId, CorrelationIdGenerator};
