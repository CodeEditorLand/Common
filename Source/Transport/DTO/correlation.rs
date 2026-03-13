//! Correlation ID types and utilities.

use serde::{Deserialize, Serialize};

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
    fn generate() -> CorrelationId;
}

/// UUID-based correlation ID generator.
pub struct UuidCorrelationIdGenerator;

impl CorrelationIdGenerator for UuidCorrelationIdGenerator {
    fn generate() -> CorrelationId {
        uuid::Uuid::new_v4().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correlation_id_generation() {
        let id1 = UuidCorrelationIdGenerator::generate();
        let id2 = UuidCorrelationIdGenerator::generate();
        assert!(!id1.is_empty());
        assert!(!id2.is_empty());
        assert_ne!(id1, id2); // Should be unique
    }
}
