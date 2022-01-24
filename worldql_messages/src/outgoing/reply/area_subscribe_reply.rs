use serde::{Deserialize, Serialize};

/// Replies to [`crate::incoming::AreaSubscribeRequest`]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct AreaSubscribeReply {
    /// Whether or not the subscription was updated
    ///
    /// Will return `true` if the client wasn't already subscribed to the area
    /// and `false` if they were
    pub updated: bool,
}

impl AreaSubscribeReply {
    /// Create a new [`AreaSubscribeReply`]
    #[inline]
    #[must_use]
    pub fn new(updated: bool) -> Self {
        Self { updated }
    }
}
