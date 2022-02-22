use serde::{Deserialize, Serialize};

use crate::common::Vector3;

/// Clear all records in a world or an area of a world
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "clear", rename_all = "snake_case")]
pub enum RecordClearRequest {
    /// Clear records by world
    ByWorld {
        /// World to clear records in
        world_name: String,
    },

    /// Clear records by area
    ByArea {
        /// World to clear records in
        world_name: String,

        /// First corner of area to clear
        pos_1: Vector3,

        /// Second corner of area to clear
        pos_2: Vector3,
    },
}

impl RecordClearRequest {
    /// Create a new [`RecordClearRequest`] to clear all records in a world
    #[inline]
    #[must_use]
    pub fn new_by_world(world_name: impl Into<String>) -> Self {
        Self::ByWorld {
            world_name: world_name.into(),
        }
    }

    /// Create a new [`RecordClearRequest`] to clear all records in an area of a world
    #[inline]
    #[must_use]
    pub fn new_by_area(
        world_name: impl Into<String>,
        pos_1: impl Into<Vector3>,
        pos_2: impl Into<Vector3>,
    ) -> Self {
        Self::ByArea {
            world_name: world_name.into(),
            pos_1: pos_1.into(),
            pos_2: pos_2.into(),
        }
    }
}
