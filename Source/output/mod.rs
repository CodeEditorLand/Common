

/**
 * @module output
 * @description This module defines the abstract contract for the Output Channel service.
 * It includes the `OutputChannelManager` trait and the `ActionEffect` constructors
 * for every output channel operation.
 */

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
mod OutputChannelManager;
pub use self::OutputChannelManager::OutputChannelManager;

// --- Effect Constructors ---
mod AppendToOutputChannel;
mod ClearOutputChannel;
mod CloseOutputChannelView;
mod DisposeOutputChannel;
mod RegisterOutputChannel;
mod ReplaceOutputChannelContent;
mod RevealOutputChannel;

pub use self::AppendToOutputChannel::AppendToOutputChannel;
pub use self::ClearOutputChannel::ClearOutputChannel;
pub use self::CloseOutputChannelView::CloseOutputChannelView;
pub use self::DisposeOutputChannel::DisposeOutputChannel;
pub use self::RegisterOutputChannel::RegisterOutputChannel;
pub use self::ReplaceOutputChannelContent::ReplaceOutputChannelContent;
pub use self::RevealOutputChannel::RevealOutputChannel;
