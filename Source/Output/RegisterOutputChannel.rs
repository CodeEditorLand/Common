//! # RegisterOutputChannel Effect
//!
//! Defines the `ActionEffect` for registering a new output channel.

use std::sync::Arc;

use super::OutputChannelManager::OutputChannelManager;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will register a new output channel
/// with the host application.
///
/// It uses the `OutputChannelManager` capability from the environment.
///
/// # Parameters
/// * `Name`: The human-readable name of the channel.
/// * `LanguageIdentifier`: An optional language ID for syntax highlighting.
///
/// # Returns
/// An `ActionEffect` that resolves with a unique `String` identifier for the
/// newly created channel.
pub fn RegisterOutputChannel<TRunTime>(
	Name:String,
	LanguageIdentifier:Option<String>,
) -> ActionEffect<Arc<TRunTime>, CommonError, String>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn OutputChannelManager>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let NameClone = Name.clone();
		let LanguageIdentifierClone = LanguageIdentifier.clone();
		Box::pin(async move {
			let Manager:Arc<dyn OutputChannelManager> = RunTime.Require();
			Manager.RegisterChannel(NameClone, LanguageIdentifierClone).await
		})
	}))
}
