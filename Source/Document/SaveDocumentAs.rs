//! # SaveDocumentAs Effect
//!
//! Defines the `ActionEffect` for saving a document to a new location.

use std::sync::Arc;

use url::Url;

use super::DocumentProvider::DocumentProvider;
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will save a document to a new
/// location. This is typically used for "Save As..." functionality.
///
/// It uses the `DocumentProvider` capability from the environment to perform
/// the operation. If `NewTargetURI` is `None`, the provider is expected to
/// interact with the user (via the `UserInterfaceProvider`) to prompt for a
/// new save location.
///
/// # Parameters
/// * `OriginalURI`: The `Url` of the document to save.
/// * `NewTargetURI`: An optional `Url` for the new save location. If `None`, a
///   save dialog should be shown to the user.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<Url>`, containing the
/// final `Url` of the saved file or `None` if the operation was cancelled.
pub fn SaveDocumentAs<TRunTime>(
	OriginalURI:Url,
	NewTargetURI:Option<Url>,
) -> ActionEffect<Arc<TRunTime>, CommonError, Option<Url>>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime: Requires<Arc<dyn DocumentProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let OriginalURIClone = OriginalURI.clone();
		let NewTargetURIClone = NewTargetURI.clone();
		Box::pin(async move {
			let Provider:Arc<dyn DocumentProvider> = RunTime.Require();
			Provider.SaveDocumentAs(OriginalURIClone, NewTargetURIClone).await
		})
	}))
}
