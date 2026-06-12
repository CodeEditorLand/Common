//! # ShowSaveDialog Effect
//!
//! Defines the `ActionEffect` for showing a native file save dialog.

use std::{path::PathBuf, sync::Arc};

use super::{DTO::SaveDialogOptionsDTO::SaveDialogOptionsDTO, UserInterfaceProvider::UserInterfaceProvider};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will display a native dialog for
/// saving a file.
///
/// Uses the `UserInterfaceProvider` capability from the environment to
/// orchestrate the interaction with the frontend.
///
/// # Parameters
/// * `Options`: An `Option<SaveDialogOptionsDTO>` containing settings for the
///   dialog, such as the title and default path.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<PathBuf>`, containing the
/// path selected by the user for saving, or `None` if the dialog was
/// cancelled.
pub fn ShowSaveDialog(
	Options:Option<SaveDialogOptionsDTO>,
) -> ActionEffect<Arc<dyn UserInterfaceProvider>, CommonError, Option<PathBuf>> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn UserInterfaceProvider>| {
		let OptionsClone = Options.clone();

		Box::pin(async move { Provider.ShowSaveDialog(OptionsClone).await })
	}))
}
