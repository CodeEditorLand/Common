//! Identifier for the calling Element. Tags every emitted event so the
//! Errors & Reliability dashboard can pivot by `$tier`.

/// Identifier for the calling Rust element (sidecar or library).
/// Each variant represents a distinct process or component in the Land
/// ecosystem. Used to tag every telemetry event so dashboards can filter
/// by source tier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
	/// The main native backend process (Tauri application).
	Mountain,

	/// The background daemon for updates and indexing.
	Air,

	/// IPC event-loop / scheduler sidecar.
	Echo,

	/// REST API sidecar.
	Rest,

	/// WebSocket / streaming sidecar.
	Grove,

	/// File-system indexing sidecar.
	Mist,

	/// Generic extension-host bridge sidecar.
	SideCar,

	/// The Common library itself (fallback when no specific tier is set).
	Common,
}

impl Tier {
	/// Returns the wire string representation of this tier.
	pub fn AsStr(&self) -> &'static str {
		match self {
			Self::Mountain => "mountain",

			Self::Air => "air",

			Self::Echo => "echo",

			Self::Rest => "rest",

			Self::Grove => "grove",

			Self::Mist => "mist",

			Self::SideCar => "sidecar",

			Self::Common => "common",
		}
	}
}
