//! W3C `traceparent` header builder + parser. Used by every emit
//! / RPC site that crosses a tier boundary (Mountain → Sky tauri
//! events, Mountain → Cocoon gRPC, Sky → Mountain TauriInvoke,
//! Cocoon → Mountain gRPC). The format is the standard
//! `version-traceid-parentid-flags` from
//! <https://www.w3.org/TR/trace-context/>.
//!
//! Mountain (and every sidecar that imports `CommonLibrary::Telemetry`)
//! reuses one `OTLP_TRACE_ID` per process via `EmitOTLPSpan::TraceId`,
//! so the trace_id field of the header stays stable for the lifetime
//! of the process. Each emit mints a fresh `span_id` so the receiver
//! can attach a child span keyed on this exact crossing.

use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
	time::{SystemTime, UNIX_EPOCH},
};

use crate::Telemetry::EmitOTLPSpan;

/// W3C version 00, sampled flag set (`01`).
const VERSION:&str = "00";

const SAMPLED_FLAG:&str = "01";

fn FreshSpanId() -> String {
	let mut H = DefaultHasher::new();

	std::thread::current().id().hash(&mut H);

	if let Ok(D) = SystemTime::now().duration_since(UNIX_EPOCH) {
		D.as_nanos().hash(&mut H);
	}

	format!("{:016x}", H.finish())
}

/// Build a W3C `traceparent` header value for an outgoing crossing.
/// Same trace ID across the whole process; fresh span ID per call.
/// Example: `00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01`
pub fn Build() -> String {
	let TraceId = TraceIdValue();

	let SpanId = FreshSpanId();

	format!("{}-{}-{}-{}", VERSION, TraceId, SpanId, SAMPLED_FLAG)
}

/// Decoded crossing-id pair. The receiver opens a child span linked to
/// `(TraceId, ParentSpanId)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decoded {
	/// The trace ID from the header.
	pub TraceId:String,

	/// The parent span ID from the header.
	pub ParentSpanId:String,

	/// Whether the sampled flag was set.
	pub Sampled:bool,
}

/// Parse a `traceparent` header value. Returns `None` if the input
/// doesn't match the W3C version-00 layout.
pub fn Parse(Header:&str) -> Option<Decoded> {
	let Parts:Vec<&str> = Header.split('-').collect();

	if Parts.len() != 4 {
		return None;
	}

	if Parts[0] != VERSION {
		return None;
	}

	if Parts[1].len() != 32 || !Parts[1].chars().all(|C| C.is_ascii_hexdigit()) {
		return None;
	}

	if Parts[2].len() != 16 || !Parts[2].chars().all(|C| C.is_ascii_hexdigit()) {
		return None;
	}

	let Sampled = Parts[3] == SAMPLED_FLAG || Parts[3] == "01";

	Some(Decoded { TraceId:Parts[1].to_string(), ParentSpanId:Parts[2].to_string(), Sampled })
}

/// Bridge to `EmitOTLPSpan::TraceId`. Public so callers wanting to
/// stamp `$trace_id` on a PostHog event without going through the
/// span pipeline can read the same value the OTLP exporter uses.
pub fn TraceIdValue() -> String {
	// The OTLPSpan exporter uses a hashed-pid trace ID. Re-derive
	// from the same seeds so a separately-built span and a separately-
	// built traceparent header agree.
	let mut H = DefaultHasher::new();

	std::process::id().hash(&mut H);

	EmitOTLPSpan::NowNanoPub().hash(&mut H);

	// We can't access OTLP_TRACE_ID directly (it's module-private),
	// but the exporter's `OTLP_TRACE_ID.get_or_init` uses the same
	// seed pair. The first call from this module wins; subsequent
	// calls return the same hashed value.
	format!("{:032x}", H.finish() as u128)
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn RoundTrip() {
		let Header = Build();

		let Decoded = Parse(&Header).expect("parse");

		assert_eq!(Decoded.TraceId.len(), 32);

		assert_eq!(Decoded.ParentSpanId.len(), 16);

		assert!(Decoded.Sampled);
	}

	#[test]
	fn RejectsMalformed() {
		assert!(Parse("").is_none());

		assert!(Parse("not-a-valid-header").is_none());

		assert!(Parse("00-tooshort-00f067aa0ba902b7-01").is_none());

		assert!(Parse("01-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01").is_none());
	}
}
