//! # RevealOutputChannel Effect
//!
//! Defines the `ActionEffect` for revealing an output channel in the UI.

use std::sync::Arc;

use super::OutputChannelManager::OutputChannelManager;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will reveal (open and focus) the
/// specified output channel in the user interface.
///
/// It uses the `OutputChannelManager` capability from the environment.
///
/// # Parameters
/// * `ChannelIdentifier`: The unique ID of the target channel.
/// * `PreserveFocus`: If `true`, the focus will remain in its current location
///   instead of moving to the output channel panel.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn RevealOutputChannel<TRunTime>(
	ChannelIdentifier:String,
	PreserveFocus:bool,
) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn OutputChannelManager>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let IdentifierClone = ChannelIdentifier.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Manager:Arc<dyn OutputChannelManager> = Environment.Require();
			Manager.Reveal(IdentifierClone, PreserveFocus).await
		})
	}))
}
