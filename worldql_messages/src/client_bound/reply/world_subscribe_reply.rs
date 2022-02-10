use serde::{Deserialize, Serialize};

/// Replies to [`WorldSubscribeRequest`](crate::server_bound::WorldSubscribeRequest)
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct WorldSubscribeReply {
    /// Whether or not the subscription was updated
    ///
    /// Will return `true` if the client wasn't already subscribed to the world
    /// and `false` if they were
    pub updated: bool,
}

impl WorldSubscribeReply {
    /// Create a new [`WorldSubscribeReply`]
    #[inline]
    #[must_use]
    pub fn new(updated: bool) -> Self {
        Self { updated }
    }
}
