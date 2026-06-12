//! # ReplaceOutputChannelContent Effect
//!
//! Defines the `ActionEffect` for replacing the entire content of an output
//! channel.

use std::sync::Arc;

use super::OutputChannelManager::OutputChannelManager;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will replace the entire buffer of
/// the specified output channel with a new string.
/// It uses the `OutputChannelManager` capability from the environment.
///
/// # Parameters
/// * `ChannelIdentifier`: The unique ID of the target channel.
/// * `Value`: The new string content for the channel.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn ReplaceOutputChannelContent(
	ChannelIdentifier:String,

	Value:String,
) -> ActionEffect<Arc<dyn OutputChannelManager>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Manager:Arc<dyn OutputChannelManager>| {
		let IdentifierClone = ChannelIdentifier.clone();

		let ValueClone = Value.clone();

		Box::pin(async move { Manager.Replace(IdentifierClone, ValueClone).await })
	}))
}
