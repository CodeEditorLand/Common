#![allow(non_snake_case)]

//! Emit `land:<tier>:session:start` with pid / OS / arch. Called once
//! by `Initialize::Fn` so the Boot & Startup Performance dashboard
//! sees one start event per sidecar process.

use crate::Telemetry::{CaptureEvent, Client};

pub fn Fn() {
	let TierStr = Client::TIER.get().map(|T| T.AsStr()).unwrap_or("common");
	let EventName = format!("land:{}:session:start", TierStr);
	let Pid = format!("{}", std::process::id());
	CaptureEvent::Fn(
		&EventName,
		Some(vec![
			("pid", Pid.as_str()),
			("os", std::env::consts::OS),
			("arch", std::env::consts::ARCH),
		]),
	);
}
