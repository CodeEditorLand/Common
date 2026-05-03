#![allow(non_snake_case)]

//! Identifier for the calling Element. Tags every emitted event so the
//! Errors & Reliability dashboard can pivot by `$tier`.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
	Mountain,
	Air,
	Echo,
	Rest,
	Grove,
	Mist,
	SideCar,
	Common,
}

impl Tier {
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
