//! # ReplaceOutputChannelContent Effect
//!
//! Defines the `ActionEffect` for replacing the entire content of an output
//! channel.

use std::sync::Arc;

use super::OutputChannelManager::OutputChannelManager;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will replace the entire buffer of
/// the specified output channel with a new string.
///
/// It uses the `OutputChannelManager` capability from the environment.
///
/// # Parameters
/// * `ChannelIdentifier`: The unique ID of the target channel.
/// * `Value`: The new string content for the channel.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn ReplaceOutputChannelContent<TRunTime>(
	ChannelIdentifier:String,
	Value:String,
) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn OutputChannelManager>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let IdentifierClone = ChannelIdentifier.clone();
		let ValueClone = Value.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Manager:Arc<dyn OutputChannelManager> = Environment.Require();
			Manager.Replace(IdentifierClone, ValueClone).await
		})
	}))
}
