//! IPC: channel registry, Sky event table, provider trait, DTOs, effect
//! constructors.

/// Enumerated IPC channel identifiers for Wind ↔ Mountain invocations.
pub mod Channel;

/// Enumerated Mountain → Sky/Wind event identifiers.
pub mod SkyEvent;

/// Abstract service trait for inter-process communication with sidecars.
pub mod IPCProvider;

/// Data Transfer Objects for IPC messages (e.g., `ProxyTarget`).
pub mod DTO;

/// Effect constructor for sending fire-and-forget notifications to a sidecar.
pub mod SendNotificationToSideCar;

/// Effect constructor for sending request-response RPC calls to a sidecar.
pub mod SendRequestToSideCar;
