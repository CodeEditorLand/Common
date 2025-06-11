// Defines the core `ActionEffect` struct, the fundamental unit of computation 
// in the application's declarative, effects-based architecture.

use std::{future::Future, pin::Pin, sync::Arc};

/// An `ActionEffect` encapsulates an asynchronous operation as a first-class
/// value.
///
/// It is a data structure that contains a function. This function, when
/// provided with its required environment (via the `RuntimeAccess` parameter),
/// produces a `Future` that will yield the result of the operation. This
/// pattern separates the *definition* of an operation from its *execution*,
/// enabling composable, testable, and declarative application logic.
///
/// # Type Parameters
/// * `RuntimeAccess`: The type of the runtime accessor the effect's
///   closure expects.
/// * `Error`: The error type that the effect's operation can return.
/// * `Output`: The success output type of the effect's operation.
pub struct ActionEffect<RuntimeAccess, Error, Output> {
	/// The wrapped asynchronous function. It is stored in an `Arc` to make the
	/// `ActionEffect` struct itself cheap to clone, as only the pointer is
	-/// copied.
	pub Function:
		Arc<dyn Fn(RuntimeAccess) -> Pin<Box<dyn Future<Output = Result<Output, Error>> + Send>> + Send + Sync>,
}

impl<RuntimeAccess, Error, Output> ActionEffect<RuntimeAccess, Error, Output> {
	/// Creates a new `ActionEffect` from a given function closure.
	pub fn New(
		Function:Arc<
			dyn Fn(RuntimeAccess) -> Pin<Box<dyn Future<Output = Result<Output, Error>> + Send>> + Send + Sync,
		>,
	) -> Self {
		Self { Function }
	}

	/// Applies the effect by executing its wrapped function with the provided
	/// runtime accessor. This is typically called by an `AppRuntime`.
	pub async fn Apply(&self, Accessor:RuntimeAccess) -> Result<Output, Error>
	where
		RuntimeAccess: Clone, {
		(self.Function)(Accessor).await
	}
}

impl<RuntimeAccess, Error, Output> Clone for ActionEffect<RuntimeAccess, Error, Output> {
	/// Clones the `ActionEffect`. This is a cheap operation as it only clones
	/// the `Arc` pointer to the underlying function, not the function itself.
	fn clone(&self) -> Self {
		ActionEffect { Function:Arc::clone(&self.Function) }
	}
}

