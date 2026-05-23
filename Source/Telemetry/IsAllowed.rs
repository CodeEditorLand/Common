//! Compile-time + runtime gates. `cfg!(debug_assertions)` strips both
//! pipes from release builds; `Capture` is the master kill, `Report` /
//! `OTLPEnabled` are per-pipe toggles. Cached after first read so the
//! hot path is one atomic load.

use std::sync::OnceLock;

use crate::Telemetry::Configuration;

static CACHED:OnceLock<Configuration::Configuration> = OnceLock::new();

fn Get() -> &'static Configuration::Configuration { CACHED.get_or_init(Configuration::Fn) }

pub fn PostHog() -> bool {
	if !cfg!(debug_assertions) {
		return false;
	}

	let C = Get();

	C.Capture && C.Report && !C.Key.is_empty()
}

pub fn OTLP() -> bool {
	if !cfg!(debug_assertions) {
		return false;
	}

	let C = Get();

	C.Capture && C.OTLPEnabled
}

pub fn Cached() -> &'static Configuration::Configuration { Get() }
