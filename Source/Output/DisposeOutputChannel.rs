//! # DisposeOutputChannel Effect
//!
//! Defines the `ActionEffect` for permanently disposing of an output channel.

use std::sync::Arc;

use super::OutputChannelManager::OutputChannelManager;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will dispose of the specified output
/// channel, removing it and its content permanently from the application.
///
/// It uses the `OutputChannelManager` capability from the environment.
///
/// # Parameters
/// * `ChannelIdentifier`: The unique ID of the target channel.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn DisposeOutputChannel(ChannelIdentifier:String) -> ActionEffect<Arc<dyn OutputChannelManager>, CommonError, ()> {
	ActionEffect::New(Arc::new(move |Manager:Arc<dyn OutputChannelManager>| {
		let IdentifierClone = ChannelIdentifier.clone();
		Box::pin(async move { Manager.Dispose(IdentifierClone).await })
	}))
}
