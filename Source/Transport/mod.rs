#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
//! # Transport Layer
//!
//! This module defines the transport-layer abstraction that enables
//! communication between CodeEditorLand components through various mechanisms
//! (gRPC, IPC, WASM) using a unified Strategy pattern interface.
//!
//! ## Architecture
//!
//! - [`TransportStrategy`] - Core trait all transports must implement
//! - [`Registry::TransportRegistry`](crate::Transport::Registry::TransportRegistry) - Dynamic transport selection and management
//! - [`UnifiedRequest`] / [`UnifiedResponse`] - Common message format
//! - [`TransportError`] - Unified error taxonomy
//! - [`TransportConfig`] - Configuration structures
//!
//! ## Sub-modules
//!
//! - [`Common`] - Shared types and utilities
//! - [`gRPC`] - gRPC transport implementation
//! - [`IPC`] - IPC (Unix sockets/Named pipes) implementation
//! - [`WASM`] - WebAssembly/WebWorker implementation
//! - [`Registry`] - Transport registry and selection
//! - [`Metrics`] - Metrics collection and monitoring
//! - [`Retry`] - Retry strategies with backoff
//! - [`CircuitBreaker`] - Circuit breaker pattern
//! - [`DTO`] - Data Transfer Objects
//!
//! ## Usage
//!
//! Components should use the transport abstraction to remain transport-agnostic:
//!
//! ```rust
//! use common_common::transport::{TransportStrategy, UnifiedRequest};
//!
//! async fn send_request(transport: &mut dyn TransportStrategy, method: &str, payload: Vec<u8>) -> Result<Vec<u8>, TransportError> {
//!     let request = UnifiedRequest::new(method, payload);
//!     let response = transport.send_request(request).await?;
//!     Ok(response.payload)
//! }
//! ```

// --- Core Trait and Types ---
pub mod TransportStrategy;
pub mod TransportError;
pub mod UnifiedRequest;
pub mod UnifiedResponse;
pub mod TransportConfig;

// --- Transport Implementations (proper acronym casing: gRPC, IPC, WASM) ---
pub mod gRPC;
pub mod IPC;
pub mod WASM;

// --- Infrastructure ---
pub mod Registry;
pub mod Metrics;
pub mod Retry;
pub mod CircuitBreaker;
pub mod Common;

// --- Data Transfer Objects ---
pub mod DTO;

