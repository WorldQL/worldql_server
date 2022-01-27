use serde::{Deserialize, Serialize};

/// Replies to [`WorldUnsubscribeRequest`](crate::incoming::WorldUnsubscribeRequest)
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct WorldUnsubscribeReply {
    /// Whether or not the subscription was updated
    ///
    /// Will return `true` if the client was subscribed to the world
    /// and `false` if they weren't
    pub updated: bool,
}

impl WorldUnsubscribeReply {
    /// Create a new [`WorldUnsubscribeReply`]
    #[inline]
    #[must_use]
    pub fn new(updated: bool) -> Self {
        Self { updated }
    }
}
