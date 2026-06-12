//! Runtime read of `.env.Land.PostHog`. Sidecars don't have their own
//! `build.rs` env-bake (Mountain does, for compile-time tree-shake) -
//! they read at boot via `std::env::var`. Mountain's `HydrateRuntime
//! Environment::Fn` populates these into the spawned process's env so
//! sidecars launched as Mountain children pick them up automatically.

/// Runtime telemetry configuration loaded from environment variables.
///
/// # Fields
/// * `Key` - PostHog API key (from `Authorize` env var).
/// * `Host` - PostHog host URL (from `Beam` env var; default `https://eu.i.posthog.com`).
/// * `Brand` - Brand identifier (from `Brand` env var).
/// * `Report` - Whether PostHog reporting is enabled (from `Report` env var).
/// * `Capture` - Master telemetry capture toggle (from `Capture` env var).
/// * `Pipe` - OTLP exporter endpoint (from `Pipe` env var; default `http://127.0.0.1:4318`).
/// * `Emit` - Whether OTLP emission is enabled (from `Emit` env var).
pub struct Configuration {
	pub Key:String,

	pub Host:String,

	pub Brand:String,

	pub Report:bool,

	pub Capture:bool,

	pub Pipe:String,

	pub Emit:bool,
}

fn ReadString(Key:&str, Fallback:&str) -> String {
	std::env::var(Key)
		.ok()
		.filter(|V| !V.is_empty())
		.unwrap_or_else(|| Fallback.to_string())
}

fn ReadBool(Key:&str, Fallback:bool) -> bool {
	match std::env::var(Key).ok().map(|V| V.to_lowercase()) {
		Some(V) => !matches!(V.as_str(), "false" | "0" | "off"),

		None => Fallback,
	}
}

/// Reads telemetry configuration from environment variables at runtime.
pub fn Fn() -> Configuration {
	Configuration {
		Key:ReadString("Authorize", ""),

		Host:ReadString("Beam", "https://eu.i.posthog.com"),

		Brand:ReadString("Brand", ""),

		Report:ReadBool("Report", false),

		Capture:ReadBool("Capture", false),

		Pipe:ReadString("Pipe", "http://127.0.0.1:4318"),

		Emit:ReadBool("Emit", false),
	}
}
