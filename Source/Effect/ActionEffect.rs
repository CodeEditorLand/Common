//! # ActionEffect Struct
//!
//! Defines the core `ActionEffect` struct, which is the fundamental unit of
//! computation in the application's declarative, effects-based architecture.

use std::{future::Future, pin::Pin, sync::Arc};

/// An `ActionEffect` encapsulates an asynchronous operation as a first-class
/// value.
///
/// It is a data structure that contains a function. This function, when
/// provided with its required environment (via the `TRunTimeAccess` parameter),
/// produces a `Future` that will yield the result of the operation. This
/// pattern cleanly separates the *definition* of an operation from its
/// *execution*, enabling composable, testable, and declarative application
/// logic.
///
/// # Type Parameters
///
/// * `TRunTimeAccess`: The type of the runtime accessor the effect's closure
///   expects.
/// * `TError`: The error type that the effect's operation can return.
/// * `TOutput`: The success output type of the effect's operation.
pub struct ActionEffect<TRunTimeAccess, TError, TOutput> {
	/// The wrapped asynchronous function. It is stored in an `Arc` to make the
	/// `ActionEffect` struct itself cheap to clone, as only the pointer is
	/// copied.
	pub Function:
		Arc<dyn Fn(TRunTimeAccess) -> Pin<Box<dyn Future<Output = Result<TOutput, TError>> + Send>> + Send + Sync>,
}

impl<TRunTimeAccess, TError, TOutput> ActionEffect<TRunTimeAccess, TError, TOutput> {
	/// Creates a new `ActionEffect` from a given function closure.
	pub fn New(
		Function:Arc<
			dyn Fn(TRunTimeAccess) -> Pin<Box<dyn Future<Output = Result<TOutput, TError>> + Send>> + Send + Sync,
		>,
	) -> Self {
		Self { Function }
	}

	/// Applies the effect by executing its wrapped function with the provided
	/// runtime accessor. This is the final step in an effect's lifecycle and is
	/// typically called by an `ApplicationRunTime`.
	pub async fn Apply(&self, Accessor:TRunTimeAccess) -> Result<TOutput, TError>
	where
		TRunTimeAccess: Clone, {
		(self.Function)(Accessor).await
	}
}

impl<TRunTimeAccess, TError, TOutput> Clone for ActionEffect<TRunTimeAccess, TError, TOutput> {
	/// Clones the `ActionEffect`.
	///
	/// This is a cheap operation as it only clones the `Arc` pointer to the
	/// underlying function, not the function itself.
	fn clone(&self) -> Self { ActionEffect { Function:Arc::clone(&self.Function) } }
}
