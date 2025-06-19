//! # ActionEffect Struct
//!
//! Defines the core `ActionEffect` struct, which is the fundamental unit of
//! computation in the application's declarative, effects-based architecture.

use std::{future::Future, pin::Pin, sync::Arc};

use super::ApplicationRunTime::ApplicationRunTime;

/// An `ActionEffect` encapsulates an asynchronous operation as a first-class
/// value.
///
/// It is a data structure that contains a function. This function, when
/// provided with its required environment (via the `TRunTime` parameter),
/// produces a `Future` that will yield the result of the operation. This
/// pattern cleanly separates the *definition* of an operation from its
/// *execution*, enabling composable, testable, and declarative application
/// logic.
///
/// # Type Parameters
///
/// * `TRunTime`: The type of the runtime the effect's closure expects.
/// * `TError`: The effect's operation.
pub struct ActionEffect<TRunTime, TError, TOutput> {
	/// The wrapped asynchronous function. It is stored in an `Arc` to make the
	/// `ActionEffect` struct itself cheap to clone, as only the pointer is
	/// copied.
	pub Function:Arc<dyn Fn(TRunTime) -> Pin<Box<dyn Future<Output = Result<TOutput, TError>> + Send>> + Send + Sync>,
}

impl<TRunTime, TError, TOutput> ActionEffect<TRunTime, TError, TOutput> {
	/// Creates a new `ActionEffect` from a given function closure.
	pub fn New(
		Function:Arc<dyn Fn(TRunTime) -> Pin<Box<dyn Future<Output = Result<TOutput, TError>> + Send>> + Send + Sync>,
	) -> Self {
		Self { Function }
	}

	/// Applies the effect by executing its wrapped function with the provided
	/// runtime. This is the final step in an effect's lifecycle and is
	/// typically called by an `ApplicationRunTime`.
	pub async fn Apply(&self, RunTime:TRunTime) -> Result<TOutput, TError>
	where
		TRunTime: Clone, {
		(self.Function)(RunTime).await
	}

	/// Transforms the output of an effect from `TOutput` to `TNewOutput`.
	pub fn map<TNewOutput, F>(self, f:F) -> ActionEffect<TRunTime, TError, TNewOutput>
	where
		TRunTime: 'static,
		TError: 'static,
		TOutput: 'static + Send,
		TNewOutput: 'static,
		F: Fn(TOutput) -> TNewOutput + Send + Sync + 'static + Copy, {
		ActionEffect::New(Arc::new(move |runtime| {
			let function = self.Function.clone();
			Box::pin(async move {
				let result = (function)(runtime).await?;
				Ok(f(result))
			})
		}))
	}
}

impl<TRunTime, TError, TOutput> Clone for ActionEffect<TRunTime, TError, TOutput> {
	/// Clones the `ActionEffect`.
	///
	/// This is a cheap operation as it only clones the `Arc` pointer to the
	/// underlying function, not the function itself.
	fn clone(&self) -> Self { ActionEffect { Function:Arc::clone(&self.Function) } }
}
