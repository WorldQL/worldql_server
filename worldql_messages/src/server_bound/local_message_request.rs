use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::common::{Replication, Vector3};

/// Broadcast a message to all clients subscribed to
/// an area inside a world
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocalMessageRequest {
    /// World to broadcast to
    pub world_name: String,

    /// Position to broadcast to
    ///
    /// Clients subscribed to the area containing this position
    /// will receive the message
    pub position: Vector3,

    /// Message replication strategy
    ///
    /// Controls the intended recipients for the message
    pub replication: Replication,

    /// Data to be broadcast
    pub data: Bytes,
}

impl LocalMessageRequest {
    /// Create a new [`LocalMessageRequest`]
    #[inline]
    #[must_use]
    pub fn new(
        world_name: impl Into<String>,
        position: Vector3,
        replication: Replication,
        data: Bytes,
    ) -> Self {
        Self {
            world_name: world_name.into(),
            position,
            replication,
            data,
        }
    }
}
