//! Process-wide PostHog client singleton. Populated once by
//! `Initialize::Fn`; every `Capture*::Fn` reads through this static.
//! Sidecars share one client per process, like Mountain's
//! `Binary/Build/PostHogPlugin/Client`.

use std::sync::OnceLock;

/// The global PostHog client instance, initialized by `Initialize::Fn`.
pub static CLIENT:OnceLock<posthog_rs::Client> = OnceLock::new();

/// The Tier that called `Initialize`, captured for default `$component`
/// + `$tier` enrichment on every event.
pub static TIER:OnceLock<crate::Telemetry::Tier::Tier> = OnceLock::new();
