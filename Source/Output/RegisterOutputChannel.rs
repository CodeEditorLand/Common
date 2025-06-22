//! # RegisterOutputChannel Effect
//!
//! Defines the `ActionEffect` for registering a new output channel.

use std::sync::Arc;

use super::OutputChannelManager::OutputChannelManager;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

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
pub fn RegisterOutputChannel(
	Name:String,

	LanguageIdentifier:Option<String>,
) -> ActionEffect<Arc<dyn OutputChannelManager>, CommonError, String> {
	ActionEffect::New(Arc::new(move |Manager:Arc<dyn OutputChannelManager>| {
		let NameClone = Name.clone();

		let LanguageIdentifierClone = LanguageIdentifier.clone();

		Box::pin(async move { Manager.RegisterChannel(NameClone, LanguageIdentifierClone).await })
	}))
}
