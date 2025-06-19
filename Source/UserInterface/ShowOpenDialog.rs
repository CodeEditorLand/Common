//! # ShowOpenDialog Effect
//!
//! Defines the `ActionEffect` for showing a native file or folder open dialog.

use std::{path::PathBuf, sync::Arc};

use super::{DTO::OpenDialogOptionsDTO::OpenDialogOptionsDTO, UserInterfaceProvider::UserInterfaceProvider};
use crate::{
	Effect::{ActionEffect::ActionEffect, ApplicationRunTime::ApplicationRunTime},
	Environment::Requires::Requires,
	Error::CommonError::CommonError,
};

/// Creates an effect that, when executed, will display a native dialog for
/// opening files or folders.
///
/// It uses the `UserInterfaceProvider` capability from the environment to
/// orchestrate the interaction with the frontend, which is responsible for
/// showing the actual OS-level dialog.
///
/// # Parameters
/// * `Options`: An `Option<OpenDialogOptionsDTO>` containing settings for the
///   dialog, such as the title, whether to allow multiple selections, and if it
///   should select folders instead of files.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<Vec<PathBuf>>`, containing
/// the list of paths selected by the user, or `None` if the dialog was
/// cancelled.
pub fn ShowOpenDialog<TRunTime>(
	Options:Option<OpenDialogOptionsDTO>,
) -> ActionEffect<Arc<TRunTime>, CommonError, Option<Vec<PathBuf>>>
where
	TRunTime: ApplicationRunTime + Send + Sync + 'static,
	TRunTime::EnvironmentType: Requires<Arc<dyn UserInterfaceProvider>>, {
	ActionEffect::New(Arc::new(move |RunTime:Arc<TRunTime>| {
		let OptionsClone = Options.clone();
		Box::pin(async move {
			let Environment = RunTime.GetEnvironment();
			let Provider:Arc<dyn UserInterfaceProvider> = Environment.Require();
			Provider.ShowOpenDialog(OptionsClone).await
		})
	}))
}
