//! # AppendToOutputChannel Effect
//!
//! Defines the `ActionEffect` for appending text to an output channel.

use std::sync::Arc;

use super::OutputChannelManager::OutputChannelManager;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

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
pub fn AppendToOutputChannel<TRunTime>(
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
			Manager.Append(IdentifierClone, ValueClone).await
		})
	}))
}
