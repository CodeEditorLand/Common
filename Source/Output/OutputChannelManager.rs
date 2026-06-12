//! # OutputChannelManager Trait
//!
//! Defines the abstract service trait for managing output channels.

use async_trait::async_trait;

use crate::{Environment::Environment::Environment, Error::CommonError::CommonError};

/// An abstract service contract for an environment component that can manage
/// output channels.
/// Output channels are a common feature in IDEs, used for displaying logs,

/// build outputs, or other textual information from extensions or system
/// processes.
#[async_trait]
pub trait OutputChannelManager: Environment + Send + Sync {
	/// Registers a new output channel with the host.
	///
	/// # Parameters
	/// * `Name`: The human-readable name of the channel to be displayed in the
	///   UI.
	/// * `LanguageIdentifier`: An optional language ID to enable syntax
	///   highlighting for the channel's content.
	///
	/// # Returns
	/// A `Result` containing a unique identifier (string) for the new channel.
	async fn RegisterChannel(&self, Name:String, LanguageIdentifier:Option<String>) -> Result<String, CommonError>;

	/// Appends a string value to the specified output channel.
	///
	/// # Parameters
	/// * `ChannelIdentifier`: The unique ID of the target channel.
	/// * `Value`: The string content to append.
	async fn Append(&self, ChannelIdentifier:String, Value:String) -> Result<(), CommonError>;

	/// Replaces the entire content of the specified output channel with a new
	/// value.
	///
	/// # Parameters
	/// * `ChannelIdentifier`: The unique ID of the target channel.
	/// * `Value`: The new string content for the channel.
	async fn Replace(&self, ChannelIdentifier:String, Value:String) -> Result<(), CommonError>;

	/// Clears all content from the specified output channel.
	async fn Clear(&self, ChannelIdentifier:String) -> Result<(), CommonError>;

	/// Reveals (opens and focuses) the specified output channel in the UI.
	///
	/// # Parameters
	/// * `ChannelIdentifier`: The ID of the channel to reveal.
	/// * `PreserveFocus`: If `true`, the focus will remain in its current
	///   location instead of moving to the output channel.
	async fn Reveal(&self, ChannelIdentifier:String, PreserveFocus:bool) -> Result<(), CommonError>;

	/// Closes the view of the specified output channel in the UI, but does not
	/// dispose of it. The channel can be revealed again later.
	async fn Close(&self, ChannelIdentifier:String) -> Result<(), CommonError>;

	/// Disposes of the specified output channel, removing it and its content
	/// permanently.
	async fn Dispose(&self, ChannelIdentifier:String) -> Result<(), CommonError>;
}
