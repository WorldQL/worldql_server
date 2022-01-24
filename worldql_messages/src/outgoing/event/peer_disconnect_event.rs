use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Emitted when a peer disconnects (or times out) from the server
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct PeerDisconnectEvent {
    /// UUID of the disconnected peer
    pub uuid: Uuid,

    /// Whether this disconnect was the result of a timeout
    pub timeout: bool
}

impl PeerDisconnectEvent {
    #[inline]
    #[must_use]
    pub fn new(uuid: Uuid, timeout: bool) -> Self {
        Self { uuid, timeout }
    }
}
