use serde::{Deserialize, Serialize};

/// Events that are not tied to a request/reply pair
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum OutgoingMessageEvent {
    // TODO
}
