use serde::{Deserialize, Serialize};

use crate::common::{PartialRecord, Vector3};

/// Lookup records by area or by ID
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "lookup", rename_all = "snake_case")]
pub enum RecordGetRequest {
    /// Lookup records by area
    Area {
        /// World containing the records
        world_name: String,

        /// Position to lookup records by
        ///
        /// Will be transformed into a region defined by the server config
        position: Vector3
    },

    /// Lookup records with known UUIDs
    Uuid {
        /// List of partial records containing information to lookup by
        records: Vec<PartialRecord>
    },
}

impl RecordGetRequest {
    #[inline]
    #[must_use]
    pub fn new_by_area(world_name: String, position: Vector3) -> Self {
        Self::Area { world_name, position }
    }

    #[inline]
    #[must_use]
    pub fn new_by_id(records: Vec<PartialRecord>) -> Self {
        Self::Uuid { records }
    }
}
