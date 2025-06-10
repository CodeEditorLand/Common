

/**
 * @module testing
 * @description This module defines the abstract contract for the Test service,
 * which is responsible for managing test controllers, test runs, and test results,
 * mirroring the `vscode.test` API.
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod TestController;
pub use self::TestController::TestController;

// --- Data Transfer Objects ---
pub mod dto;

// --- Effect Constructors ---
mod RegisterTestController;
mod RunTests;

pub use self::RegisterTestController::RegisterTestController;
pub use self::RunTests::RunTests;
