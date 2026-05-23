//! Stable distinct ID for the dev session. Mirrors Mountain's
//! `Binary/Build/PostHogPlugin/DistinctId` so a single dev run merges
//! into one PostHog person across every sidecar.

use crate::Telemetry::IsAllowed;

pub fn Fn() -> String {
	let Brand = &IsAllowed::Cached().Brand;

	if !Brand.is_empty() {
		return Brand.clone();
	}

	let User = std::env::var("USER")
		.or_else(|_| std::env::var("USERNAME"))
		.unwrap_or_else(|_| "unknown".to_string());

	format!("land-dev-{}", User)
}
