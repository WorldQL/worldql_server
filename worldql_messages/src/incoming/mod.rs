use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use self::heartbeat_request::HeartbeatRequest;

mod heartbeat_request;

// region: IncomingMessage
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IncomingMessage {
    /// UUID of the Client that sent this message
    pub sender: Uuid,

    /// Unique token to verify the client's UUID is correct and not being spoofed
    pub token: String,

    /// Message payload
    pub payload: IncomingMessagePayload,
}

impl IncomingMessage {
    pub fn new(sender: Uuid, token: String, payload: IncomingMessagePayload) -> Self {
        Self {
            sender,
            token,
            payload,
        }
    }
}
// endregion

// region: IncomingMessagePayload
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "request", rename_all = "snake_case")]
pub enum IncomingMessagePayload {
    Heartbeat(HeartbeatRequest),
    // TODO
}
// endregion

// region: IntoIncomingMessage Trait
pub trait IntoIncomingMessage {
    /// Convert an [`IncomingMessagePayload`] into an [`IncomingMessage`]
    #[must_use]
    fn into_incoming_message(self, sender: Uuid, token: String) -> IncomingMessage;
}

impl<T: Into<IncomingMessagePayload>> IntoIncomingMessage for T {
    #[inline]
    fn into_incoming_message(self, sender: Uuid, token: String) -> IncomingMessage {
        IncomingMessage {
            sender,
            token,
            payload: self.into(),
        }
    }
}
// endregion
