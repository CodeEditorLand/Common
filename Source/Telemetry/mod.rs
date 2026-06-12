//! # Telemetry - shared dual-pipe (PostHog + OTLP) for every Rust Element
//!
//! Single source of truth for the sidecar telemetry surface. Mountain
//! keeps its own compile-time-baked `Binary/Build/PostHogPlugin/*`
//! (the build.rs `cargo:rustc-env` route guarantees release builds drop
//! every byte). All other Rust elements (Air, Echo, Rest, Grove, Mist,
//! SideCar) read configuration at runtime through this module:
//!
//! ```ignore
//! use CommonLibrary::Telemetry::{Initialize, Tier};
//! Initialize::Fn(Tier::Air);   // call once during boot
//! ```
//!
//! All capture functions are no-ops in release builds
//! (`cfg!(debug_assertions)`) and short-circuit on `Capture=false` / per-pipe
//! toggles at runtime.
//!
//! ## Layout (one export per file)
//!
//! - `Tier::Tier` - enum identifying the calling sidecar
//! - `Configuration::Fn` - runtime env read
//! - `IsAllowed::PostHog`, `IsAllowed::OTLP` - gates
//! - `Client::CLIENT` - `OnceLock<posthog_rs::Client>`
//! - `DistinctId::Fn` - stable per-machine identity
//! - `CaptureEvent::Fn`, `CaptureError::Fn`, `CaptureSession::Fn` - PostHog
//! - `EmitOTLPSpan::Fn` - raw HTTP OTLP exporter
//! - `Initialize::Fn` - `Tier`-tagged boot

/// PostHog error capture function.
pub mod CaptureError;

/// PostHog event capture function.
pub mod CaptureEvent;

/// PostHog session capture function.
pub mod CaptureSession;

/// `OnceLock<posthog_rs::Client>` singleton.
pub mod Client;

/// Runtime telemetry configuration reader.
pub mod Configuration;

/// Stable per-machine identity provider.
pub mod DistinctId;

/// Raw HTTP OTLP span exporter.
pub mod EmitOTLPSpan;

/// Telemetry initialization (call once during boot).
pub mod Initialize;

/// Per-pipe gate checks (`PostHog`, `OTLP`, `Cached`).
pub mod IsAllowed;

/// Enum identifying the calling sidecar (Air, Echo, Rest, etc.).
pub mod Tier;

/// W3C Traceparent header for distributed tracing.
pub mod Traceparent;
