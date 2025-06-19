//! # WriteFileString Effect
//!
//! Defines a convenience `ActionEffect` for writing string content to a file.

use std::{path::PathBuf, sync::Arc};

use super::WriteFileBytes::WriteFileBytes;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Error::CommonError::CommonError,
};

/// Creates a convenience effect that writes string content to a file.
///
/// This function is a wrapper around `WriteFileBytes`. It first converts the
/// provided `String` into a byte vector (`Vec<u8>`) and then delegates to the
/// `WriteFileBytes` effect constructor. This simplifies call sites that are
/// working with text data.
///
/// # Parameters
/// * `Path`: The `PathBuf` of the file to write to.
/// * `Content`: The `String` content to be written.
/// * `Create`: If `true`, the file will be created if it does not exist.
/// * `Overwrite`: If `true`, an existing file will be overwritten.
///
/// # Returns
/// An `ActionEffect` that resolves to `()` on success.
pub fn WriteFileString<TRunTime>(
	Path:PathBuf,
	Content:String,
	Create:bool,
	Overwrite:bool,
) -> ActionEffect<Arc<TRunTime>, CommonError, ()>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static, {
	WriteFileBytes(Path, Content.into_bytes(), Create, Overwrite)
}
