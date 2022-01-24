use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// Broadcast a message to all clients subscribed to a world
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalMessageRequest {
    /// World to broadcast to
    ///
    /// Use the special world `@global` to broadcast to all clients
    pub world_name: String,

    /// Data to be broadcast
    pub data: Bytes,
}

impl GlobalMessageRequest {
    /// Create a new [`GlobalMessageRequest`]
    #[inline]
    #[must_use]
    pub fn new(world_name: impl Into<String>, data: Bytes) -> Self {
        Self { world_name: world_name.into(), data }
    }
}
