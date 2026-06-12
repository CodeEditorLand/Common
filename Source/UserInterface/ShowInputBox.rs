//! # ShowInputBox Effect
//!
//! Defines the `ActionEffect` for showing an input box to the user.

use std::sync::Arc;

use super::{DTO::InputBoxOptionsDTO::InputBoxOptionsDTO, UserInterfaceProvider::UserInterfaceProvider};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will display an input box to solicit
/// a string input from the user.
/// It uses the `UserInterfaceProvider` capability from the environment to
/// orchestrate the interaction with the frontend UI.
///
/// # Parameters
/// * `Options`: An `Option<InputBoxOptionsDTO>` containing settings for the
///   input box, such as a title, placeholder text, and initial value.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<String>`, containing the
/// text entered by the user, or `None` if the input box was cancelled.
pub fn ShowInputBox(
	Options:Option<InputBoxOptionsDTO>,
) -> ActionEffect<Arc<dyn UserInterfaceProvider>, CommonError, Option<String>> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn UserInterfaceProvider>| {
		let OptionsClone = Options.clone();

		Box::pin(async move { Provider.ShowInputBox(OptionsClone).await })
	}))
}
