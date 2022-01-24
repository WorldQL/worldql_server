use serde::{Deserialize, Serialize};

use crate::common::Vector3;

/// Clear all records in a world or an area of a world
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordClearRequest {
    /// World to clear records in
    world_name: String,

    /// Optional position, if set will only clear records in that region
    ///
    /// Will be transformed into a region defined by the server config
    position: Option<Vector3>,
}

impl RecordClearRequest {
    #[inline]
    #[must_use]
    pub fn new_clear_world(world_name: String) -> Self {
        Self {
            world_name,
            position: None,
        }
    }

    #[inline]
    #[must_use]
    pub fn new_clear_world_area(world_name: String, position: Vector3) -> Self {
        Self {
            world_name,
            position: Some(position),
        }
    }
}
