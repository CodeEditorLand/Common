//! Stable distinct ID for the dev session. Mirrors Mountain's
//! `Binary/Build/PostHogPlugin/DistinctId` so a single dev run merges
//! into one PostHog person across every sidecar.

use crate::Telemetry::IsAllowed;

/// Generates a stable distinct ID for the current session.
///
/// Uses the `Brand` from cached configuration if non-empty, otherwise
/// falls back to `land-dev-{USER}` derived from the `USER` or `USERNAME`
/// environment variable.
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
