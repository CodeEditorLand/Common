//! # ShowInputBox Effect
//!
//! Defines the `ActionEffect` for showing an input box to the user.

use std::sync::Arc;

use super::{DTO::InputBoxOptionsDTO::InputBoxOptionsDTO, UserInterfaceProvider::UserInterfaceProvider};
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will display an input box to solicit
/// a string input from the user.
///
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
pub fn ShowInputBox<TRunTime>(
	Options:Option<InputBoxOptionsDTO>,
) -> ActionEffect<Arc<TRunTime>, CommonError, Option<String>>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn UserInterfaceProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let OptionsClone = Options.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Provider:Arc<dyn UserInterfaceProvider> = Environment.Require();
			Provider.ShowInputBox(OptionsClone).await
		})
	}))
}
