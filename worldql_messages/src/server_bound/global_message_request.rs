use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::common::Replication;

/// Broadcast a message to all clients subscribed to a world
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalMessageRequest {
    /// World to broadcast to
    ///
    /// Use the special world `@global` to broadcast to all clients
    pub world_name: String,

    /// Message replication strategy
    ///
    /// Controls the intended recipients for the message
    pub replication: Replication,

    /// Data to be broadcast
    pub data: Bytes,

    /// Whether or not to send a reply acknowledging the request
    pub ack: bool,
}

impl GlobalMessageRequest {
    /// Create a new [`GlobalMessageRequest`]
    #[inline]
    #[must_use]
    pub fn new(
        world_name: impl Into<String>,
        replication: Replication,
        data: Bytes,
        ack: bool,
    ) -> Self {
        Self {
            world_name: world_name.into(),
            replication,
            data,
            ack,
        }
    }
}
