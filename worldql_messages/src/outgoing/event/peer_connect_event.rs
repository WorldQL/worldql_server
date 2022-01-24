use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Emitted when a new peer fully connects to the server
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct PeerConnectEvent {
    /// UUID of the newly connected peer
    pub uuid: Uuid,
}

impl PeerConnectEvent {
    #[inline]
    #[must_use]
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid }
    }
}
