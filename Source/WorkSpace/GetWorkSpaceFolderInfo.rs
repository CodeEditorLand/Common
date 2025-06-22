//! # GetWorkSpaceFolderInfo Effect
//!
//! Defines the `ActionEffect` for retrieving information about the specific
//! workspace folder that contains a given URI.

use std::sync::Arc;

use url::Url;

use super::WorkSpaceProvider::WorkSpaceProvider;
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will find the workspace folder that
/// contains the given URI.
///
/// It uses the `WorkSpaceProvider` capability from the environment. This is
/// useful for determining which folder-level settings apply to a specific
/// file.
///
/// # Parameters
/// * `URIToMatch`: The `Url` of the resource (e.g., a file) for which to find
///   the containing workspace folder.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option` containing a tuple of the
/// folder's `Url`, name, and index, or `None` if the URI is not within any
/// open workspace folder.
pub fn GetWorkSpaceFolderInfo(
	URIToMatch:Url,
) -> ActionEffect<Arc<dyn WorkSpaceProvider>, CommonError, Option<(Url, String, usize)>> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn WorkSpaceProvider>| {
		let URIClone = URIToMatch.clone();

		Box::pin(async move { Provider.GetWorkSpaceFolderInfo(URIClone).await })
	}))
}
