//! # TestController Trait
//!
//! Defines the abstract service trait for managing test controllers and test
//! execution, mirroring the `vscode.test` API.

use async_trait::async_trait;
use serde_json::Value;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// test providers and test runs contributed by extensions.
#[async_trait]
pub trait TestController: Environment + Send + Sync {
	/// Registers a new test controller from a sidecar (extension host).
	///
	/// # Parameters
	/// * `ControllerId`: A unique identifier for the test controller.
	/// * `Label`: A human-readable label for the test controller.
	async fn RegisterTestController(&self, ControllerId:String, Label:String) -> Result<(), CommonError>;

	/// Runs a set of tests.
	///
	/// # Parameters
	/// * `ControllerId`: The ID of the controller that owns the tests to be
	///   run.
	/// * `TestRunRequest`: A DTO representing the request, including which
	///   specific tests to run (or all if omitted) and whether it's a debug
	///   run.
	async fn RunTests(&self, ControllerId:String, TestRunRequest:Value) -> Result<(), CommonError>;
}
