//! # Output Service
//!
//! This module defines the abstract contract for the Output Channel service.
//! It includes the `OutputChannelManager` trait and the `ActionEffect`
//! constructors for every output channel operation.

#![allow(non_snake_case, non_camel_case_types)]

// --- Trait Definition ---
pub mod OutputChannelManager;

// --- Effect Constructors ---
pub mod AppendToOutputChannel;
pub mod ClearOutputChannel;
pub mod CloseOutputChannelView;
pub mod DisposeOutputChannel;
pub mod RegisterOutputChannel;
pub mod ReplaceOutputChannelContent;
pub mod RevealOutputChannel;
