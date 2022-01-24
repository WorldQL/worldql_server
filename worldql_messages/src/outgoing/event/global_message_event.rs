use bytes::Bytes;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Incoming Global Messages
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
    #[inline]
    #[must_use]
    pub fn new(sender: Uuid, world_name: String, data: Bytes) -> Self {
        Self {
            sender,
            world_name,
            data,
        }
    }
}
