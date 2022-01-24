use bytes::Bytes;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::Vector3;

/// Incoming Local Message
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocalMessageEvent {
    /// UUID of the client that triggered this event
    pub sender: Uuid,

    /// World that is being broadcast to
    pub world_name: String,

    /// Position of the local message
    pub position: Vector3,

    /// Message data
    pub data: Bytes,
}

impl LocalMessageEvent {
    #[inline]
    #[must_use]
    pub fn new(sender: Uuid, world_name: String, position: Vector3, data: Bytes) -> Self {
        Self {
            sender,
            world_name,
            position,
            data,
        }
    }
}
