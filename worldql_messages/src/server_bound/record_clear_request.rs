use serde::{Deserialize, Serialize};

use crate::common::Vector3;

/// Clear all records in a world or an area of a world
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordClearRequest {
    /// World to clear records in
    pub world_name: String,

    /// Optional position, if set will only clear records in that region
    ///
    /// Will be transformed into a region defined by the server config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Vector3>,
}

impl RecordClearRequest {
    /// Create a new [`RecordClearRequest`] to clear all records in a world
    #[inline]
    #[must_use]
    pub fn new_clear_world(world_name: impl Into<String>) -> Self {
        Self {
            world_name: world_name.into(),
            position: None,
        }
    }

    /// Create a new [`RecordClearRequest`] to clear all records in an area of a world
    #[inline]
    #[must_use]
    pub fn new_clear_world_area(world_name: impl Into<String>, position: Vector3) -> Self {
        Self {
            world_name: world_name.into(),
            position: Some(position),
        }
    }
}
