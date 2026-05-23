//! Sidecar boot. Idempotent: subsequent calls are no-ops because
//! `OnceLock::set` returns `Err`. Pass the `Tier` so every emitted
//! event is tagged correctly without per-call boilerplate.
//!
//! Usage:
//! ```ignore
//! use CommonLibrary::Telemetry::{Initialize, Tier};
//!
//! #[tokio::main]
//! async fn main() {
//!     Initialize::Fn(Tier::Tier::Air).await;
//!     // ... rest of sidecar boot ...
//! }
//! ```

use crate::Telemetry::{CaptureSession, Client, IsAllowed, Tier};

pub async fn Fn(Tier:Tier::Tier) {
	let _ = Client::TIER.set(Tier);

	if !IsAllowed::PostHog() {
		return;
	}

	let Configuration = IsAllowed::Cached();

	let Options = match posthog_rs::ClientOptionsBuilder::default()
		.api_key(Configuration.Key.clone())
		.host(Configuration.Host.clone())
		.build()
	{
		Ok(O) => O,

		Err(_) => return,
	};

	let PostHogClient = posthog_rs::client(Options).await;

	let _ = Client::CLIENT.set(PostHogClient);

	CaptureSession::Fn();
}
