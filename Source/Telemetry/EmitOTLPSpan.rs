#![allow(non_snake_case)]

//! Fire-and-forget OTLP span exporter. Lifted from Mountain's
//! `IPC/DevLog/EmitOTLPSpan` so Air / Echo / Rest / Grove / Mist /
//! SideCar all share the same raw HTTP path. Single failed POST flips
//! `OTLP_AVAILABLE` to false so a missing collector doesn't tax every
//! emit. Release builds compile out via `cfg!(debug_assertions)`.

use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
	sync::{
		OnceLock,
		atomic::{AtomicBool, Ordering},
	},
	time::{SystemTime, UNIX_EPOCH},
};

use crate::Telemetry::{Client, IsAllowed};

static OTLP_AVAILABLE:AtomicBool = AtomicBool::new(true);

static OTLP_TRACE_ID:OnceLock<String> = OnceLock::new();

fn NowNano() -> u64 {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.map(|D| D.as_nanos() as u64)
		.unwrap_or(0)
}

fn TraceId() -> &'static str {
	OTLP_TRACE_ID.get_or_init(|| {
		let mut H = DefaultHasher::new();
		std::process::id().hash(&mut H);
		NowNano().hash(&mut H);
		format!("{:032x}", H.finish() as u128)
	})
}

fn RandU64() -> u64 {
	let mut H = DefaultHasher::new();

	std::thread::current().id().hash(&mut H);

	NowNano().hash(&mut H);

	H.finish()
}

fn ParseEndpoint(Endpoint:&str) -> (String, String) {
	let WithoutScheme = Endpoint
		.strip_prefix("http://")
		.or_else(|| Endpoint.strip_prefix("https://"))
		.unwrap_or(Endpoint);

	let (HostPort, Path) = match WithoutScheme.split_once('/') {
		Some((HP, Rest)) => (HP.to_string(), format!("/{}", Rest.trim_start_matches('/'))),

		None => (WithoutScheme.to_string(), "/v1/traces".to_string()),
	};

	let PathFinal = if Path == "/" { "/v1/traces".to_string() } else { Path };

	(HostPort, PathFinal)
}

/// Emit one span. `StartNano` / `EndNano` are wall-clock (not monotonic)
/// nanosecond timestamps - use `NowNano()` from the caller's start.
pub fn Fn(Name:&str, StartNano:u64, EndNano:u64, Attributes:&[(&str, &str)]) {
	if !IsAllowed::OTLP() {
		return;
	}

	if !OTLP_AVAILABLE.load(Ordering::Relaxed) {
		return;
	}

	let Configuration = IsAllowed::Cached();

	let TierStr = Client::TIER.get().map(|T| T.AsStr()).unwrap_or("common");

	let SpanId = format!("{:016x}", RandU64());

	let TraceIdString = TraceId().to_string();

	let SpanName = Name.to_string();

	let AttributesJson:Vec<String> = Attributes
		.iter()
		.map(|(K, V)| {
			format!(
				r#"{{"key":"{}","value":{{"stringValue":"{}"}}}}"#,
				K,
				V.replace('\\', "\\\\").replace('"', "\\\"")
			)
		})
		.collect();

	let IsError = SpanName.contains("error");

	let StatusCode = if IsError { 2 } else { 1 };

	let ServiceName = format!("land-editor-{}", TierStr);

	let Payload = format!(
		concat!(
			r#"{{"resourceSpans":[{{"resource":{{"attributes":["#,
			r#"{{"key":"service.name","value":{{"stringValue":"{}"}}}},"#,
			r#"{{"key":"service.version","value":{{"stringValue":"0.0.1"}}}},"#,
			r#"{{"key":"land.tier","value":{{"stringValue":"{}"}}}}"#,
			r#"]}},"scopeSpans":[{{"scope":{{"name":"land.{}","version":"1.0.0"}},"#,
			r#""spans":[{{"traceId":"{}","spanId":"{}","name":"{}","kind":1,"#,
			r#""startTimeUnixNano":"{}","endTimeUnixNano":"{}","#,
			r#""attributes":[{}],"status":{{"code":{}}}}}]}}]}}]}}"#,
		),
		ServiceName,
		TierStr,
		TierStr,
		TraceIdString,
		SpanId,
		SpanName,
		StartNano,
		EndNano,
		AttributesJson.join(","),
		StatusCode,
	);

	let (HostAddress, PathSegment) = ParseEndpoint(&Configuration.OTLPEndpoint);

	std::thread::spawn(move || {
		use std::{
			io::{Read as IoRead, Write as IoWrite},
			net::TcpStream,
			time::Duration,
		};

		let Ok(SocketAddress) = HostAddress.parse() else {
			OTLP_AVAILABLE.store(false, Ordering::Relaxed);
			return;
		};
		let Ok(mut Stream) = TcpStream::connect_timeout(&SocketAddress, Duration::from_millis(200)) else {
			OTLP_AVAILABLE.store(false, Ordering::Relaxed);
			return;
		};
		let _ = Stream.set_write_timeout(Some(Duration::from_millis(200)));
		let _ = Stream.set_read_timeout(Some(Duration::from_millis(200)));

		let HttpReq = format!(
			"POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: \
			 close\r\n\r\n",
			PathSegment,
			HostAddress,
			Payload.len()
		);
		if Stream.write_all(HttpReq.as_bytes()).is_err() {
			return;
		}
		if Stream.write_all(Payload.as_bytes()).is_err() {
			return;
		}
		let mut Buf = [0u8; 32];
		let _ = Stream.read(&mut Buf);
		if !(Buf.starts_with(b"HTTP/1.1 2") || Buf.starts_with(b"HTTP/1.0 2")) {
			OTLP_AVAILABLE.store(false, Ordering::Relaxed);
		}
	});
}

/// Helper exposed to callers that need a span window timestamp.
pub fn NowNanoPub() -> u64 { NowNano() }
