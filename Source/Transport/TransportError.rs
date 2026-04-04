#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
//! # TransportError
//!
//! Defines the unified error type for all transport operations.

use std::fmt;

use super::TransportStrategy::TransportErrorCode;

/// Unified transport error.
#[derive(Debug)]
pub struct TransportError {
	/// Error code indicating the type of failure.
	pub Code: TransportErrorCode,

	/// Human-readable error message.
	pub Message: String,

	/// Optional underlying/boxed error.
	pub Source: Option<Box<dyn std::error::Error + Send + Sync>>,

	/// The transport type that generated this error.
	pub TransportKind: String,

	/// The method being invoked when the error occurred (if applicable).
	pub Method: Option<String>,

	/// The correlation/request ID for tracing.
	pub CorrelationIdentifier: Option<String>,

	/// Number of retry attempts before this failure.
	pub RetryAttempt: u32,

	/// Additional error context as key-value pairs.
	pub Context: std::collections::HashMap<String, String>,
}

impl TransportError {
	/// Creates a new `TransportError` with the given code and message.
	pub fn New(Code: TransportErrorCode, Message: impl Into<String>) -> Self {
		Self {
			Code,
			Message: Message.into(),
			Source: None,
			TransportKind: String::new(),
			Method: None,
			CorrelationIdentifier: None,
			RetryAttempt: 0,
			Context: std::collections::HashMap::new(),
		}
	}

	/// Sets the transport type on this error.
	pub fn WithTransportKind(mut self, TransportKind: &str) -> Self {
		self.TransportKind = TransportKind.to_string();
		self
	}

	/// Sets the method name on this error.
	pub fn WithMethod(mut self, Method: &str) -> Self {
		self.Method = Some(Method.to_string());
		self
	}

	/// Sets the correlation/request ID on this error.
	pub fn WithCorrelationIdentifier(mut self, CorrelationIdentifier: &str) -> Self {
		self.CorrelationIdentifier = Some(CorrelationIdentifier.to_string());
		self
	}

	/// Sets the retry attempt count.
	pub fn WithRetryAttempt(mut self, RetryAttempt: u32) -> Self {
		self.RetryAttempt = RetryAttempt;
		self
	}

	/// Adds a context key-value pair to this error.
	pub fn WithContext(mut self, Key: &str, Value: &str) -> Self {
		self.Context.insert(Key.to_string(), Value.to_string());
		self
	}

	/// Sets the underlying source error.
	pub fn WithSource(
		mut self,
		SourceError: impl std::error::Error + Send + Sync + 'static,
	) -> Self {
		self.Source = Some(Box::new(SourceError));
		self
	}

	/// Returns `true` if this error is retryable.
	pub fn IsRetryable(&self) -> bool {
		self.Code.IsRetryable()
	}

	/// Returns the recommended retry delay in milliseconds.
	pub fn RetryDelayMilliseconds(&self) -> u64 {
		self.Code.RecommendedRetryDelayMilliseconds()
	}

	/// Returns the full error message with all context included.
	pub fn FullMessage(&self) -> String {
		let mut MessageText = self.Message.clone();

		if let Some(Method) = &self.Method {
			MessageText.push_str(&format!(" (method: {})", Method));
		}

		if let Some(CorrelationIdentifier) = &self.CorrelationIdentifier {
			MessageText.push_str(&format!(" (correlation_id: {})", CorrelationIdentifier));
		}

		if !self.TransportKind.is_empty() {
			MessageText.push_str(&format!(" (transport: {})", self.TransportKind));
		}

		if self.RetryAttempt > 0 {
			MessageText.push_str(&format!(" (retry: {})", self.RetryAttempt));
		}

		if !self.Context.is_empty() {
			let ContextString = self
				.Context
				.iter()
				.map(|(Key, Value)| format!("{}={}", Key, Value))
				.collect::<Vec<_>>()
				.join(", ");
			MessageText.push_str(&format!(" (context: {{{}}})", ContextString));
		}

		if let Some(SourceError) = &self.Source {
			MessageText.push_str(&format!(" (cause: {})", SourceError));
		}

		MessageText
	}
}

impl Clone for TransportError {
	fn clone(&self) -> Self {
		Self {
			Code: self.Code,
			Message: self.Message.clone(),
			Source: None,
			TransportKind: self.TransportKind.clone(),
			Method: self.Method.clone(),
			CorrelationIdentifier: self.CorrelationIdentifier.clone(),
			RetryAttempt: self.RetryAttempt,
			Context: self.Context.clone(),
		}
	}
}

impl PartialEq for TransportError {
	fn eq(&self, Other: &Self) -> bool {
		self.Code == Other.Code
			&& self.Message == Other.Message
			&& self.TransportKind == Other.TransportKind
			&& self.Method == Other.Method
			&& self.CorrelationIdentifier == Other.CorrelationIdentifier
			&& self.RetryAttempt == Other.RetryAttempt
			&& self.Context == Other.Context
	}
}

impl Eq for TransportError {}

impl fmt::Display for TransportError {
	fn fmt(&self, Formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(Formatter, "{}", self.FullMessage())
	}
}

impl std::error::Error for TransportError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		self.Source
			.as_ref()
			.map(|SourceError| SourceError.as_ref() as &dyn std::error::Error)
	}
}

/// Convenience constructors for common transport errors.
impl TransportError {
	/// Connection error: failed to connect or lost connection.
	pub fn Connection(Message: impl Into<String>) -> Self {
		Self::New(TransportErrorCode::ConnectionFailed, Message).WithTransportKind("unknown")
	}

	/// Timeout error: operation exceeded deadline.
	pub fn Timeout(Message: impl Into<String>) -> Self {
		Self::New(TransportErrorCode::Timeout, Message).WithTransportKind("unknown")
	}

	/// Invalid request error: bad parameters or format.
	pub fn InvalidRequest(Message: impl Into<String>) -> Self {
		Self::New(TransportErrorCode::InvalidRequest, Message)
	}

	/// Not supported error: feature not implemented by this transport.
	pub fn NotSupported(Message: impl Into<String>) -> Self {
		Self::New(TransportErrorCode::NotSupported, Message)
	}

	/// Remote error: the remote endpoint returned an error.
	pub fn Remote(Message: impl Into<String>) -> Self {
		Self::New(TransportErrorCode::RemoteError, Message)
	}

	/// Internal error: something went wrong inside the transport.
	pub fn Internal(Message: impl Into<String>) -> Self {
		Self::New(TransportErrorCode::InternalError, Message)
	}

	/// Circuit breaker open error: request rejected due to circuit breaker.
	pub fn CircuitBreakerOpen() -> Self {
		Self::New(TransportErrorCode::CircuitBreakerOpen, "Circuit breaker is open")
			.WithTransportKind("unknown")
	}

	/// Rate limited error: too many requests.
	pub fn RateLimited(RetryAfterMilliseconds: u64) -> Self {
		let mut Error =
			Self::New(TransportErrorCode::RateLimited, "Rate limit exceeded").WithContext(
				"retry_after_ms",
				&RetryAfterMilliseconds.to_string(),
			);
		Error
			.Context
			.insert("retry_after".to_string(), format!("{}ms", RetryAfterMilliseconds));
		Error
	}

	/// Message too large error.
	pub fn MessageTooLarge(Size: usize, MaximumSize: usize) -> Self {
		Self::New(
			TransportErrorCode::MessageTooLarge,
			format!("Message size {} exceeds maximum {}", Size, MaximumSize),
		)
		.WithContext("size", &Size.to_string())
		.WithContext("max_size", &MaximumSize.to_string())
	}

	/// Not found error: resource or transport not found.
	pub fn NotFound(Message: impl Into<String>) -> Self {
		Self::New(TransportErrorCode::NotFound, Message)
	}

	/// Serialization error.
	pub fn Serialization(Message: impl Into<String>) -> Self {
		Self::New(TransportErrorCode::SerializationError, Message)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn TestTransportErrorConstruction() {
		let Error = TransportError::Connection("Connection refused");
		assert_eq!(Error.Code, TransportErrorCode::ConnectionFailed);
		assert!(Error.Message.contains("Connection refused"));
	}

	#[test]
	fn TestErrorContext() {
		let Error = TransportError::New(TransportErrorCode::Timeout, "Request timed out")
			.WithMethod("ping")
			.WithCorrelationIdentifier("12345")
			.WithContext("endpoint", "localhost:50051");

		assert_eq!(Error.Method, Some("ping".to_string()));
		assert_eq!(Error.CorrelationIdentifier, Some("12345".to_string()));
		assert_eq!(
			Error.Context.get("endpoint"),
			Some(&"localhost:50051".to_string())
		);
	}

	#[test]
	fn TestErrorIsRetryable() {
		let ConnectionError = TransportError::Connection("Connection failed");
		assert!(ConnectionError.IsRetryable());

		let InvalidError = TransportError::InvalidRequest("Bad params");
		assert!(!InvalidError.IsRetryable());
	}

	#[test]
	fn TestErrorFullMessage() {
		let Error = TransportError::Timeout("Operation timed out")
			.WithMethod("get_file")
			.WithCorrelationIdentifier("abc-123")
			.WithTransportKind("grpc");

		let FullMessage = Error.FullMessage();
		assert!(FullMessage.contains("Operation timed out"));
		assert!(FullMessage.contains("method: get_file"));
		assert!(FullMessage.contains("correlation_id: abc-123"));
		assert!(FullMessage.contains("transport: grpc"));
	}
}
