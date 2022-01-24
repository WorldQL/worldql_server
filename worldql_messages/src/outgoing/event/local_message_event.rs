use bytes::Bytes;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::Vector3;

/// Emitted when a local message is triggered for an
/// area that the client is subscribed to
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
    /// Create a new [`LocalMessageEvent`]
    #[inline]
    #[must_use]
    pub fn new(sender: Uuid, world_name: impl Into<String>, position: Vector3, data: Bytes) -> Self {
        Self {
            sender,
            world_name: world_name.into(),
            position,
            data,
        }
    }
}
