

/**
 * @module ipc
 * @description This module defines the abstract contract for the Inter-Process
 * Communication (IPC) service. It includes the `IpcProvider` trait, all related
 * DTOs, and the `ActionEffect` constructors for all IPC operations.
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod IpcProvider;
pub use self::IpcProvider::IpcProvider;

// --- Data Transfer Objects ---
pub mod dto;

// --- Effect Constructors ---
mod EstablishHostConnection;
mod ProxyCallToSidecar;
mod SendNotificationToSidecar;
mod SendRequestToSidecar;

pub use self::EstablishHostConnection::EstablishHostConnection;
pub use self::ProxyCallToSidecar::ProxyCallToSidecar;
pub use self::SendNotificationToSidecar::SendNotificationToSidecar;
pub use self::SendRequestToSidecar::SendRequestToSidecar;
