use serde::{Deserialize, Serialize};

/// Unsubscribe from messages for a world
///
/// Will also clear local message area subscriptions
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorldUnsubscribeRequest {
    /// World to unsubscribe from
    pub world_name: String,
}

impl WorldUnsubscribeRequest {
    /// Create a new [`WorldUnsubscribeRequest`]
    #[inline]
    #[must_use]
    pub fn new(world_name: impl Into<String>) -> Self {
        Self {
            world_name: world_name.into(),
        }
    }
}
