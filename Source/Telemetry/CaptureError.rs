//! Emit `land:<tier>:error` with `error_tag` + `error_message`. The
//! Errors & Reliability dashboard rolls these up across Mountain /
//! Cocoon / Sky / sidecars via `event LIKE 'land:%:error'`.

use crate::Telemetry::{CaptureEvent, Client};

/// Captures a PostHog error event tagged with the current tier.
///
/// The event is emitted as `land:<tier>:error` with `error_tag` and
/// `error_message` properties. Respects the global telemetry gate
/// (`IsAllowed::PostHog`), so this is a no-op when telemetry is disabled.
///
/// # Parameters
/// * `Tag` - A short error category tag (e.g., `"io_error"`,
///   `"parse_failure"`).
/// * `Message` - A human-readable error description.
pub fn Fn(Tag:&str, Message:&str) {
	let TierStr = Client::TIER.get().map(|T| T.AsStr()).unwrap_or("common");

	let EventName = format!("land:{}:error", TierStr);

	CaptureEvent::Fn(&EventName, Some(vec![("error_tag", Tag), ("error_message", Message)]));
}
