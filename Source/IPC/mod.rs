//! # IPC Service
//!
//! This module defines the abstract contract for the Inter-Process
//! Communication (IPC) service. It includes the `IPCProvider` trait, which
//! specifies the methods for communicating with external processes, all
//! related Data Transfer Objects (DTOs), and the `ActionEffect` constructors
//! for all IPC operations.

// --- Channel Registry (Wind ↔ Mountain Tauri invoke table) ---
pub mod Channel;

// --- Sky Event Registry (Mountain → Sky/Wind Tauri event table) ---
pub mod SkyEvent;

// --- Trait Definition ---
pub mod IPCProvider;

// --- Data Transfer Objects ---
pub mod DTO;

// --- Effect Constructors ---
pub mod EstablishHostConnection;

pub mod ProxyCallToSideCar;

pub mod SendNotificationToSideCar;

pub mod SendRequestToSideCar;
