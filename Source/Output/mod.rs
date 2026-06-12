//! # Output Service
//!
//! This module defines the abstract contract for the Output Channel service.
//! It includes the `OutputChannelManager` trait and the `ActionEffect`
//! constructors for every output channel operation.

// --- Trait Definition ---
/// Trait for managing output channels.
pub mod OutputChannelManager;

// --- Effect Constructors ---
/// Effect constructor for appending text to an output channel.
pub mod AppendToOutputChannel;

/// Effect constructor for clearing an output channel.
pub mod ClearOutputChannel;

/// Effect constructor for closing an output channel's view.
pub mod CloseOutputChannelView;

/// Effect constructor for disposing an output channel.
pub mod DisposeOutputChannel;

/// Effect constructor for registering a new output channel.
pub mod RegisterOutputChannel;

/// Effect constructor for replacing an output channel's content.
pub mod ReplaceOutputChannelContent;

/// Effect constructor for revealing (focusing) an output channel.
pub mod RevealOutputChannel;
