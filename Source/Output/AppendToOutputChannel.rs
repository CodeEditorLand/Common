//! # AppendToOutputChannel Effect
//!
//! Defines the `ActionEffect` for appending text to an output channel.

use std::sync::Arc;

use super::OutputChannelManager::OutputChannelManager;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will append a string to the
/// specified output channel's buffer.
///
/// It uses the `OutputChannelManager` capability from the environment.
///
/// # Parameters
/// * `ChannelIdentifier`: The unique ID of the target channel.
/// * `Value`: The string content to append.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn AppendToOutputChannel(
	ChannelIdentifier:String,

	Value:String,
) -> ActionEffect<Arc<dyn OutputChannelManager>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Manager:Arc<dyn OutputChannelManager>| {
		let IdentifierClone = ChannelIdentifier.clone();

		let ValueClone = Value.clone();

		Box::pin(async move { Manager.Append(IdentifierClone, ValueClone).await })
	}))
}
