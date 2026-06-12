//! # RevealOutputChannel Effect
//!
//! Defines the `ActionEffect` for revealing an output channel in the UI.

use std::sync::Arc;

use super::OutputChannelManager::OutputChannelManager;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will reveal (open and focus) the
/// specified output channel in the user interface.
/// It uses the `OutputChannelManager` capability from the environment.
///
/// # Parameters
/// * `ChannelIdentifier`: The unique ID of the target channel.
/// * `PreserveFocus`: If `true`, the focus will remain in its current location
///   instead of moving to the output channel panel.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn RevealOutputChannel(
	ChannelIdentifier:String,

	PreserveFocus:bool,
) -> ActionEffect<Arc<dyn OutputChannelManager>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Manager:Arc<dyn OutputChannelManager>| {
		let IdentifierClone = ChannelIdentifier.clone();

		Box::pin(async move { Manager.Reveal(IdentifierClone, PreserveFocus).await })
	}))
}
