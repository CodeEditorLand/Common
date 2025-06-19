//! # ShowMessage Effect
//!
//! Defines the `ActionEffect` for displaying a modal message to the user.

use std::sync::Arc;

use serde_json::Value;

use super::{DTO::MessageSeverity::MessageSeverity, UserInterfaceProvider::UserInterfaceProvider};
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will display a message to the user
/// with a given severity and a set of optional action buttons.
///
/// It uses the `UserInterfaceProvider` capability from the environment to
/// perform the operation, which typically involves sending an event to the
/// frontend and waiting for the user's interaction.
///
/// # Parameters
/// * `Severity`: The `MessageSeverity` of the message (Info, Warning, Error).
/// * `Message`: The primary text content of the message.
/// * `OptionsValue`: A `serde_json::Value` representing the
///   `MessageOptionsDTO`, which can include a title, detail text, and a list of
///   action buttons.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<String>`, containing the
/// string title of the action button the user clicked, or `None` if the
/// message was dismissed without an action.
pub fn ShowMessage<TRunTime>(
	Severity:MessageSeverity,
	Message:String,
	OptionsValue:Value,
) -> ActionEffect<Arc<TRunTime>, CommonError, Option<String>>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn UserInterfaceProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let MessageClone = Message.clone();
		let OptionsClone = OptionsValue.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Provider:Arc<dyn UserInterfaceProvider> = Environment.Require();
			Provider.ShowMessage(Severity, MessageClone, Some(OptionsClone)).await
		})
	}))
}
