//! Compile-time + runtime gates. `cfg!(debug_assertions)` strips both
//! pipes from release builds; `Capture` is the master kill, `Report` /
//! `Emit` are per-pipe toggles. Cached after first read so the
//! hot path is one atomic load.

use std::sync::OnceLock;

use crate::Telemetry::Configuration;

static CACHED:OnceLock<Configuration::Configuration> = OnceLock::new();

fn Get() -> &'static Configuration::Configuration { CACHED.get_or_init(Configuration::Fn) }

/// Returns `true` if the PostHog pipe is enabled (debug build + `Capture`
/// + `Report` + non-empty API key).
pub fn PostHog() -> bool {
	if !cfg!(debug_assertions) {
		return false;
	}

	let C = Get();

	C.Capture && C.Report && !C.Key.is_empty()
}

/// Returns `true` if the OTLP pipe is enabled (debug build + `Capture` +
/// `Emit`).
pub fn OTLP() -> bool {
	if !cfg!(debug_assertions) {
		return false;
	}

	let C = Get();

	C.Capture && C.Emit
}

/// Returns the cached telemetry configuration, initializing it on first
/// call via `Configuration::Fn()`.
pub fn Cached() -> &'static Configuration::Configuration { Get() }
