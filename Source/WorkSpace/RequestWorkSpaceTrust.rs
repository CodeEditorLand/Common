//! # RequestWorkSpaceTrust Effect
//!
//! Defines the `ActionEffect` for prompting the user to trust the workspace.

use std::sync::Arc;

use serde_json::Value;

use super::WorkSpaceProvider::WorkSpaceProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will prompt the user to grant or
/// deny trust to the current workspace via a UI dialog.
///
/// It uses the `WorkSpaceProvider` capability from the environment, which in
/// turn will likely use the `UserInterfaceProvider` to show the dialog.
///
/// # Parameters
/// * `Options`: An optional `serde_json::Value` that can contain additional
///   information or options for the trust prompt.
///
/// # Returns
/// An `ActionEffect` that resolves with a `bool` indicating whether trust was
/// granted.
pub fn RequestWorkSpaceTrust(Options:Option<Value>) -> ActionEffect<Arc<dyn WorkSpaceProvider>, CommonError, bool> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn WorkSpaceProvider>| {
		let OptionsClone = Options.clone();
		Box::pin(async move { Provider.RequestWorkSpaceTrust(OptionsClone).await })
	}))
}
