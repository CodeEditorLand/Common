//! Emit a named PostHog event. Stamps the calling `Tier` plus the
//! standard Land identity (`$app`, `$app_version`, `$build_mode`,
//! `$component`, `$tier`) so dashboards can pivot by tier without
//! caller changes.

use crate::Telemetry::{Client, DistinctId, IsAllowed};

/// Captures a named PostHog event with standard Land identity properties.
/// Automatically stamps every event with `$app`, `$app_version`,
/// `$build_mode`, `$component`, `$tier`, and `$lib`. Respects the
/// `IsAllowed::PostHog()` gate - this is a no-op when telemetry is
/// disabled.
///
/// # Parameters
/// * `EventName` - The name of the event to emit (e.g.,
///   `"land:echo:session:start"`).
/// * `Properties` - Optional key-value pairs of additional event properties.
pub fn Fn(EventName:&str, Properties:Option<Vec<(&str, &str)>>) {
	if !IsAllowed::PostHog() {
		return;
	}

	let Some(C) = Client::CLIENT.get() else { return };

	let TierStr = Client::TIER.get().map(|T| T.AsStr()).unwrap_or("common");

	let mut Event = posthog_rs::Event::new(EventName, &DistinctId::Fn());

	let _ = Event.insert_prop("$app", "fiddee");

	let _ = Event.insert_prop("$app_version", "0.0.1");

	let _ = Event.insert_prop("$build_mode", "debug");

	let _ = Event.insert_prop("$component", TierStr);

	let _ = Event.insert_prop("$tier", TierStr);

	let _ = Event.insert_prop("$lib", "land-common-telemetry");

	if let Some(Props) = Properties {
		for (Key, Value) in Props {
			let _ = Event.insert_prop(Key, Value);
		}
	}

	let _ = C.capture(Event);
}
