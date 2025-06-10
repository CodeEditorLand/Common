use std::{path::PathBuf, sync::Arc};

/// @module WriteFileString
/// @description Defines a convenience ActionEffect for writing string content
/// to a file.
use super::WriteFileBytes::WriteFileBytes;
use crate::{
	effect::{ActionEffect, AppRuntime},
	error::CommonError,
};

/// Creates a convenience effect that writes string content to a file.
///
/// This function is a wrapper around `WriteFileBytes`. It first converts the
/// provided `String` into a byte vector (`Vec<u8>`) and then delegates to the
/// `WriteFileBytes` effect constructor.
///
/// @param Path - The `PathBuf` of the file to write to.
/// @param Content - The `String` content to be written.
/// @param Create - If `true`, the file will be created if it does not exist.
/// @param Overwrite - If `true`, an existing file will be overwritten.
///
/// @returns An `ActionEffect` that resolves to `()` on success.
pub fn WriteFileString<Runtime>(
	Path:PathBuf,
	Content:String,
	Create:bool,
	Overwrite:bool,
) -> ActionEffect<Arc<Runtime>, CommonError, ()>
where
	Runtime: AppRuntime + Send + Sync + 'static, {
	WriteFileBytes(Path, Content.into_bytes(), Create, Overwrite)
}
