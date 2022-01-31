use serde::{Deserialize, Serialize};

/// Replication strategy for global and local messages
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Replication {
    /// Emit the message to all subscribed clients excluding the sender
    ExceptSelf,

    /// Emit the message to all subscribed clients including the sender
    IncludingSelf,

    /// Only deliver the message to the sender, provided they are subscribed
    OnlySelf,
}

impl Default for Replication {
    fn default() -> Self {
        Self::ExceptSelf
    }
}
