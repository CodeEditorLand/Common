// File: Common/Source/UserInterface/ShowQuickPick.rs
// Role: Defines the `ShowQuickPick` ActionEffect.
// Responsibilities:
//   - Provide a declarative effect for showing a quick pick list to the user.
//   - This effect abstracts the "what" (show a quick pick) from the "how" (the
//     UserInterfaceProvider implementation).

//! # ShowQuickPick Effect
//!
//! Defines the `ActionEffect` for showing a quick pick list to the user.

use std::sync::Arc;

use super::{
	DTO::{QuickPickItemDTO::QuickPickItemDTO, QuickPickOptionsDTO::QuickPickOptionsDTO},
	UserInterfaceProvider::UserInterfaceProvider,
};
use crate::{Effect::ActionEffect::ActionEffect, Error::CommonError::CommonError};

/// Creates an effect that, when executed, will display a quick pick list to
/// the user, allowing them to select one or more items.
///
/// It uses the `UserInterfaceProvider` capability from the environment to
/// orchestrate the interaction with the frontend UI.
///
/// # Parameters
/// * `Items`: A `Vec<QuickPickItemDTO>` representing the list of items to
///   display.
/// * `Options`: An `Option<QuickPickOptionsDTO>` containing settings for the
///   quick pick, such as the title and whether multiple selections are allowed.
///
/// # Returns
/// An `ActionEffect` that resolves with an `Option<Vec<String>>`, containing
/// the labels of the items selected by the user, or `None` if the quick pick
/// was cancelled.
pub fn ShowQuickPick(
	Items:Vec<QuickPickItemDTO>,
	Options:Option<QuickPickOptionsDTO>,
) -> ActionEffect<Arc<dyn UserInterfaceProvider>, CommonError, Option<Vec<String>>> {
	ActionEffect::New(Arc::new(move |Provider:Arc<dyn UserInterfaceProvider>| {
		let ItemsClone = Items.clone();
		let OptionsClone = Options.clone();
		Box::pin(async move { Provider.ShowQuickPick(ItemsClone, OptionsClone).await })
	}))
}
