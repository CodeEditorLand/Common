//! # WASM Transport Implementation
//!
//! Channel-based transport for the Grove WebAssembly extension host.
//! Because WASM has no sockets, message passing is done through a
//! tokio mpsc channel pair created on Connect.
