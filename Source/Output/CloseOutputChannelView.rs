//! # CloseOutputChannelView Effect
//!
//! Defines the `ActionEffect` for closing the view of an output channel in the
//! UI.

use std::sync::Arc;

use super::OutputChannelManager::OutputChannelManager;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will close the view of the specified
/// output channel in the UI.
///
/// This does not dispose of the channel or its content; it can be revealed
/// again later. It uses the `OutputChannelManager` capability from the
/// environment.
///
/// # Parameters
/// * `ChannelIdentifier`: The unique ID of the target channel.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn CloseOutputChannelView<TRunTime>(ChannelIdentifier:String) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn OutputChannelManager>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let IdentifierClone = ChannelIdentifier.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Manager:Arc<dyn OutputChannelManager> = Environment.Require();
			Manager.Close(IdentifierClone).await
		})
	}))
}
