use bytes::Bytes;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Emitted when a global message is triggered on
/// a world that the client is subscribed to
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalMessageEvent {
    /// UUID of the client that triggered this event
    pub sender: Uuid,

    /// World that is being broadcast to
    pub world_name: String,

    /// Message data
    pub data: Bytes,
}

impl GlobalMessageEvent {
    /// Create a new [`GlobalMessageEvent`]
    #[inline]
    #[must_use]
    pub fn new(sender: Uuid, world_name: impl Into<String>, data: Bytes) -> Self {
        Self {
            sender,
            world_name: world_name.into(),
            data,
        }
    }
}
