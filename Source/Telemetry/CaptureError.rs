#![allow(non_snake_case)]

//! Emit `land:<tier>:error` with `error_tag` + `error_message`. The
//! Errors & Reliability dashboard rolls these up across Mountain /
//! Cocoon / Sky / sidecars via `event LIKE 'land:%:error'`.

use crate::Telemetry::{CaptureEvent, Client};

pub fn Fn(Tag:&str, Message:&str) {
	let TierStr = Client::TIER.get().map(|T| T.AsStr()).unwrap_or("common");
	let EventName = format!("land:{}:error", TierStr);
	CaptureEvent::Fn(&EventName, Some(vec![("error_tag", Tag), ("error_message", Message)]));
}
