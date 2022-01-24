use serde::{Deserialize, Serialize};

/// Replies to [`crate::incoming::AreaUnsubscribeRequest`]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct AreaUnsubscribeReply {
    /// Whether or not the subscription was updated
    ///
    /// Will return `true` if the client was subscribed to the area
    /// and `false` if they weren't
    pub updated: bool,
}

impl AreaUnsubscribeReply {
    /// Create a new [`AreaUnsubscribeReply`]
    #[inline]
    #[must_use]
    pub fn new(updated: bool) -> Self {
        Self { updated }
    }
}
