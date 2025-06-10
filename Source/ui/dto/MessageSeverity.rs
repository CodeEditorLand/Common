use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageSeverity {
	Info,
	Warning,
	Error,
}
