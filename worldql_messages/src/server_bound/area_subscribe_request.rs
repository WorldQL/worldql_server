use serde::{Deserialize, Serialize};

use crate::common::Vector3;

/// Subscribe to an area in a world to receive local messages
///
/// Implies subscribing to global messages for this world,
/// if not already subscribed
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AreaSubscribeRequest {
    /// World to subscribe to
    pub world_name: String,

    /// Position for the subscription
    ///
    /// Will be transformed into a region defined by the server config
    pub position: Vector3,
}

impl AreaSubscribeRequest {
    /// Create a new [`AreaSubscribeRequest`]
    #[inline]
    #[must_use]
    pub fn new(world_name: impl Into<String>, position: impl Into<Vector3>) -> Self {
        Self {
            world_name: world_name.into(),
            position: position.into(),
        }
    }
}
