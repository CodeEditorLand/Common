//! # MessageSeverity DTO
//!
//! Defines the Data Transfer Object enum for representing the severity level
//! of a user-facing message.

use serde::{Deserialize, Serialize};

/// An enum representing the severity of a message to be shown to the user.
/// This controls the visual style (e.g., icon, color) of the notification.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageSeverity {
	Info,
	Warning,
	Error,
}
