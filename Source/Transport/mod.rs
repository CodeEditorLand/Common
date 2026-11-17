//! # Transport Layer
//!
//! Defines the transport-layer abstraction that enables communication between
//! CodeEditorLand components through various mechanisms (gRPC, IPC, WASM)
//! using a unified Strategy pattern interface.
//!
//! ## Architecture
//!
//! - [`TransportStrategy`] - Core trait all transports must implement
//! - [`Registry::TransportRegistry`] - Dynamic transport selection and
//!   management
//! - [`UnifiedRequest`] / [`UnifiedResponse`] - Common message format
//! - [`TransportError`] - Unified error taxonomy
//! - [`TransportConfig`] - Configuration structures
//!
//! ## Sub-modules
//!
//! - [`TransportStrategy`] - Core trait all transports must implement
//! - [`TransportError`] - Unified error taxonomy
//! - [`UnifiedRequest`] - Common request message format
//! - [`UnifiedResponse`] - Common response message format
//! - [`TransportConfig`] - Configuration structures
//! - [`Common`] - Shared types and utilities
//!
//! ## Usage
//!
//! Components should use the transport abstraction to remain
//! transport-agnostic:
//!
//! ```rust
//! use common_common::transport::{TransportStrategy, UnifiedRequest};
//!
//! async fn send_request(
//! 	transport:&mut dyn TransportStrategy,
//! 	method:&str,
//! 	payload:Vec<u8>,
//! ) -> Result<Vec<u8>, TransportError> {
//! 	let request = UnifiedRequest::new(method, payload);
//! 	let response = transport.send_request(request).await?;
//! 	Ok(response.payload)
//! }
//! ```

// --- Core Trait and Types ---
pub mod TransportStrategy;

pub mod TransportError;

pub mod UnifiedRequest;

pub mod UnifiedResponse;

pub mod TransportConfig;

pub mod Common;
