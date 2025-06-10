

/**
 * @module diagnostics
 * @description This module defines the abstract contract for the Diagnostics service,
 * which is responsible for managing problems like errors and warnings. It includes
 * the `DiagnosticsManager` trait and the `ActionEffect` constructors for all
 * diagnostic operations.
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod DiagnosticsManager;
pub use self::DiagnosticsManager::DiagnosticsManager;

// --- Effect Constructors ---
mod ClearDiagnostics;
mod GetAllDiagnostics;
mod SetDiagnostics;

pub use self::ClearDiagnostics::ClearDiagnostics;
pub use self::GetAllDiagnostics::GetAllDiagnostics;
pub use self::SetDiagnostics::SetDiagnostics;
