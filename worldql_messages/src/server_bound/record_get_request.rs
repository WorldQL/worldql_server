use serde::{Deserialize, Serialize};

use crate::common::{PartialRecord, Vector3};

/// Lookup records by area or by ID
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "lookup", rename_all = "snake_case")]
pub enum RecordGetRequest {
    /// Lookup records by area
    ByArea {
        /// World containing the records
        world_name: String,

        /// First corner of area to lookup
        pos_1: Vector3,

        /// Second corner of area to lookup
        pos_2: Vector3,
    },

    /// Lookup records with known UUIDs
    ByUuid {
        /// List of partial records containing information to lookup by
        records: Vec<PartialRecord>,
    },
}

impl RecordGetRequest {
    /// Create a new [`RecordGetRequest`] to lookup by area
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

    /// Create a new [`RecordGetRequest`] to lookup by UUID
    #[inline]
    #[must_use]
    pub fn new_by_id(records: Vec<PartialRecord>) -> Self {
        Self::ByUuid { records }
    }
}
