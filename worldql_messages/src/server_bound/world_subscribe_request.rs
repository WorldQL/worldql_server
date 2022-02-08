use serde::{Deserialize, Serialize};

/// Subscribe to global messages for a world
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorldSubscribeRequest {
    /// World to subscribe to
    pub world_name: String,
}

impl WorldSubscribeRequest {
    /// Create a new [`WorldSubscribeRequest`]
    #[inline]
    #[must_use]
    pub fn new(world_name: impl Into<String>) -> Self {
        Self {
            world_name: world_name.into(),
        }
    }
}
