use serde::{Deserialize, Serialize};
use uuid::Uuid;

// region: IncomingMessage
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IncomingMessage {
    pub sender: Uuid,
    pub token: String,
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
