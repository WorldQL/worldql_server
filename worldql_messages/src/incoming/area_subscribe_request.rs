use serde::{Deserialize, Serialize};

use crate::common::Vector3;

/// Subscribe to an area in a world to recieve local messages
///
/// Implies subscribing to global messages for this world,
/// if not already subscribed
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AreaSubscribeRequest {
    /// World to subscribe to
    pub world_name: String,

    /// Position for the subscription
    ///
    /// Will be transformed into an region defined by the server config
    pub position: Vector3,
}

impl AreaSubscribeRequest {
    #[inline]
    #[must_use]
    pub fn new(world_name: String, position: Vector3) -> Self {
        Self {
            world_name,
            position,
        }
    }
}
