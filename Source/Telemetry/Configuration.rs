//! Runtime read of `.env.Land.PostHog`. Sidecars don't have their own
//! `build.rs` env-bake (Mountain does, for compile-time tree-shake) -
//! they read at boot via `std::env::var`. Mountain's `HydrateRuntime
//! Environment::Fn` populates these into the spawned process's env so
//! sidecars launched as Mountain children pick them up automatically.

pub struct Configuration {
	pub Key:String,

	pub Host:String,

	pub Brand:String,

	pub Report:bool,

	pub Capture:bool,

	pub OTLPEndpoint:String,

	pub OTLPEnabled:bool,
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

pub fn Fn() -> Configuration {
	Configuration {
		Key:ReadString("Authorize", ""),

		Host:ReadString("Beam", "https://eu.i.posthog.com"),

		Brand:ReadString("Brand", ""),

		Report:ReadBool("Report", false),

		Capture:ReadBool("Capture", false),

		OTLPEndpoint:ReadString("OTLPEndpoint", "http://127.0.0.1:4318"),

		OTLPEnabled:ReadBool("OTLPEnabled", false),
	}
}
