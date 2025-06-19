//! # IPC Service
//!
//! This module defines the abstract contract for the Inter-Process
//! Communication (IPC) service. It includes the `IPCProvider` trait, which
//! specifies the methods for communicating with external processes, all
//! related Data Transfer Objects (DTOs), and the `ActionEffect` constructors
//! for all IPC operations.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod IPCProvider;
// pub use self::IPCProvider::IPCProvider;

// --- Data Transfer Objects ---
pub mod DTO;

// --- Effect Constructors ---
pub mod EstablishHostConnection;
pub mod ProxyCallToSidecar;
pub mod SendNotificationToSidecar;
pub mod SendRequestToSidecar;

// pub use self::{
// 	EstablishHostConnection::EstablishHostConnection,
// 	ProxyCallToSidecar::ProxyCallToSidecar,
// 	SendNotificationToSidecar::SendNotificationToSidecar,
// 	SendRequestToSidecar::SendRequestToSidecar,
// };
