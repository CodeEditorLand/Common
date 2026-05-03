#![allow(non_snake_case)]

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

pub mod CaptureError;
pub mod CaptureEvent;
pub mod CaptureSession;
pub mod Client;
pub mod Configuration;
pub mod DistinctId;
pub mod EmitOTLPSpan;
pub mod Initialize;
pub mod IsAllowed;
pub mod Tier;
