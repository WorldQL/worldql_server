use serde::{Deserialize, Serialize};


/// Responses to [`crate::incoming::IncomingMessage`] requests
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "reply", rename_all = "snake_case")]
pub enum OutgoingMessageReply {
    // TODO
}
