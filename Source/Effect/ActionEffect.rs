//! # ActionEffect Struct
//!
//! Defines the core `ActionEffect` struct, which is the fundamental unit of
//! computation in the application's declarative, effects-based architecture.

use std::{future::Future, pin::Pin, sync::Arc};

/// An `ActionEffect` encapsulates an asynchronous operation as a first-class
/// value.
///
/// It is a data structure that contains a function. This function, when
/// provided with its required capability (`TCapability`), produces a `Future`
/// that will yield the result of the operation. This pattern cleanly separates
/// the *definition* of an operation from its *execution*.
///
/// # Type Parameters
///
/// * `TCapability`: The type of the capability (e.g., `Arc<dyn
///   FileSystemReader>`) that the effect's closure requires to run.
/// * `TError`: The error type that the effect's operation can return.
/// * `TOutput`: The success output type of the effect's operation.
pub struct ActionEffect<TCapability, TError, TOutput> {
	/// The wrapped asynchronous function. It is stored in an `Arc` to make the
	/// `ActionEffect` struct itself cheap to clone.
	pub Function:
		Arc<dyn Fn(TCapability) -> Pin<Box<dyn Future<Output = Result<TOutput, TError>> + Send>> + Send + Sync>,
}

impl<TCapability, TError, TOutput> ActionEffect<TCapability, TError, TOutput> {
	/// Creates a new `ActionEffect` from a given function closure.
	pub fn New(
		Function:Arc<
			dyn Fn(TCapability) -> Pin<Box<dyn Future<Output = Result<TOutput, TError>> + Send>> + Send + Sync,
		>,
	) -> Self {
		Self { Function }
	}

	/// Applies the effect by executing its wrapped function with the provided
	/// capability. This is typically called by an `ApplicationRunTime`.
	pub async fn Apply(&self, Capability:TCapability) -> Result<TOutput, TError>
	where
		TCapability: Clone, {
		(self.Function)(Capability).await
	}

	/// Transforms the output of an effect from `TOutput` to `TNewOutput`.
	pub fn map<TNewOutput, F>(self, _Function:F) -> ActionEffect<TCapability, TError, TNewOutput>
	where
		TCapability: 'static + Send,
		TError: 'static,
		TOutput: 'static + Send,
		TNewOutput: 'static,
		F: Fn(TOutput) -> TNewOutput + Send + Sync + 'static + Copy, {
		ActionEffect::New(Arc::new(move |Capability| {
			let Function = self.Function.clone();

			Box::pin(async move {
				let Result = (Function)(Capability).await?;

				Ok(_Function(Result))
			})
		}))
	}
}

impl<TCapability, TError, TOutput> Clone for ActionEffect<TCapability, TError, TOutput> {
	/// Clones the `ActionEffect`. This is a cheap operation as it only clones
	/// the `Arc` pointer to the underlying function.
	fn clone(&self) -> Self { ActionEffect { Function:Arc::clone(&self.Function) } }
}
