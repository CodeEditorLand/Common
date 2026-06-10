//! Correlation ID types and utilities.

/// Correlation ID type.
///
/// Used to uniquely identify requests and match responses to requests.
pub type CorrelationId = String;

/// Trait for generating correlation IDs.
///
/// This allows different ID generation strategies (UUID, sequential, etc.)
/// for testing or special requirements.
pub trait CorrelationIdGenerator {
	/// Generates a new unique correlation ID.
	fn Generate() -> CorrelationId;
}

/// UUID-based correlation ID generator.
pub struct UuidCorrelationIdGenerator;

impl CorrelationIdGenerator for UuidCorrelationIdGenerator {
	fn Generate() -> CorrelationId { uuid::Uuid::new_v4().to_string() }
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn TestCorrelationIdGeneration() {
		let Identifier1 = UuidCorrelationIdGenerator::Generate();

		let Identifier2 = UuidCorrelationIdGenerator::Generate();

		assert!(!Identifier1.is_empty());

		assert!(!Identifier2.is_empty());

		assert_ne!(Identifier1, Identifier2);
	}
}
