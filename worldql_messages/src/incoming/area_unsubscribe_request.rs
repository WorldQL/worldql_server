use serde::{Deserialize, Serialize};

use crate::common::Vector3;

/// Unsubscribe from an area in a world
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AreaUnsubscribeRequest {
    /// World to unsubscribe from
    pub world_name: String,

    /// Position for the subscription
    ///
    /// Will be transformed into a region defined by the server config
    pub position: Vector3,
}

impl AreaUnsubscribeRequest {
    /// Create a new [`AreaUnsubscribeRequest`]
    #[inline]
    #[must_use]
    pub fn new(world_name: String, position: Vector3) -> Self {
        Self {
            world_name,
            position,
        }
    }
}
