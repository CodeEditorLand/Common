// File: Common/Source/OutputEffect.rs
// Responsibility: Responsibility could not be determined.
// Modified: 2025-06-04 00:37:32 UTC

// Land_Common/src/output_effects.rs
use std::sync::Arc;

use async_trait::async_trait;

// Ensure AppRuntime is the correct type from your runtime module.
// This accessor is used by effects to get to the concrete environment.
use crate::runtime::AppRuntime;
use crate::{
	effect::ActionEffect,
	environment::{Environment, Requires},
	errors::CommonError,
};
// No serde_json::Value needed for these method signatures directly,
// but the environment implementation might use it for event payloads.

/// Trait for managing output channels in the application.
///
/// Output channels are typically used for displaying logs, build outputs,
/// or other textual information from extensions or system processes.
#[async_trait]
pub trait OutputChannelManager: Environment {
	/// Registers a new output channel with the given name.
	/// Optionally, a language ID can be associated with the channel for syntax
	/// highlighting. Returns a unique ID for the registered channel (often the
	/// name itself).
	async fn register_channel(&self, name:String, language_id:Option<String>) -> Result<String, CommonError>;

	/// Appends a string value to the specified output channel.
	async fn append(&self, channel_id:String, value:String) -> Result<(), CommonError>;

	/// Replaces the entire content of the specified output channel with the
	/// given string value.
	async fn replace(&self, channel_id:String, value:String) -> Result<(), CommonError>;

	/// Clears all content from the specified output channel.
	async fn clear(&self, channel_id:String) -> Result<(), CommonError>;

	/// Reveals (opens and focuses) the specified output channel in the UI.
	async fn reveal(&self, channel_id:String, preserve_focus:bool) -> Result<(), CommonError>;

	/// Closes the view of the specified output channel in the UI, but does not
	/// dispose of it. The channel and its content remain available.
	async fn close(&self, channel_id:String) -> Result<(), CommonError>;

	/// Disposes of the specified output channel, removing it and its content
	/// permanently.
	async fn dispose(&self, channel_id:String) -> Result<(), CommonError>;
}

// --- Effect Constructors ---

/// Creates an effect to register an output channel.
pub fn register_output_channel(
	name:String,
	language_id:Option<String>,
) -> ActionEffect<Arc<AppRuntime>, CommonError, String> {
	// Effect expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let name_clone = name.clone();
		let lang_id_clone = language_id.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let manager:Arc<dyn OutputChannelManager + Send + Sync> = concrete_env.require();
			manager.register_channel(name_clone, lang_id_clone).await
		})
	}))
}

/// Creates an effect to append text to an output channel.
pub fn append_to_output_channel(channel_id:String, value:String) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Effect expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let cid_clone = channel_id.clone();
		let val_clone = value.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let manager:Arc<dyn OutputChannelManager + Send + Sync> = concrete_env.require();
			manager.append(cid_clone, val_clone).await
		})
	}))
}

/// Creates an effect to replace the content of an output channel.
pub fn replace_output_channel_content(
	channel_id:String,
	value:String,
) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Effect expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let cid_clone = channel_id.clone();
		let val_clone = value.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let manager:Arc<dyn OutputChannelManager + Send + Sync> = concrete_env.require();
			manager.replace(cid_clone, val_clone).await
		})
	}))
}

/// Creates an effect to clear an output channel.
pub fn clear_output_channel(channel_id:String) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Effect expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let cid_clone = channel_id.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let manager:Arc<dyn OutputChannelManager + Send + Sync> = concrete_env.require();
			manager.clear(cid_clone).await
		})
	}))
}

/// Creates an effect to reveal an output channel.
pub fn reveal_output_channel(channel_id:String, preserve_focus:bool) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Effect expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let cid_clone = channel_id.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let manager:Arc<dyn OutputChannelManager + Send + Sync> = concrete_env.require();
			manager.reveal(cid_clone, preserve_focus).await
		})
	}))
}

/// Creates an effect to close the view of an output channel.
pub fn close_output_channel_view(channel_id:String) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Effect expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let cid_clone = channel_id.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let manager:Arc<dyn OutputChannelManager + Send + Sync> = concrete_env.require();
			manager.close(cid_clone).await
		})
	}))
}

/// Creates an effect to dispose of an output channel.
pub fn dispose_output_channel(channel_id:String) -> ActionEffect<Arc<AppRuntime>, CommonError, ()> {
	// Effect expects Arc<AppRuntime>
	ActionEffect::new(Arc::new(move |app_runtime_accessor:Arc<AppRuntime>| {
		let cid_clone = channel_id.clone();
		Box::pin(async move {
			let concrete_env = app_runtime_accessor.get_environment();
			let manager:Arc<dyn OutputChannelManager + Send + Sync> = concrete_env.require();
			manager.dispose(cid_clone).await
		})
	}))
}
