use bytes::Bytes;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Vector3;

// region: Record
/// Data stored in a position in 3D space inside a world
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Record {
    /// Unique identifier for this record
    ///
    /// **Must be globally unique**
    pub uuid: Uuid,

    /// World to store this record in
    pub world_name: String,

    /// Position of this record
    pub position: Vector3,

    /// Optional data held by this record
    pub data: Option<Bytes>,
}

impl Record {
    /// Create a new [`Record`]
    #[inline]
    #[must_use]
    pub fn new(uuid: Uuid, world_name: String, position: Vector3, data: Option<Bytes>) -> Self {
        Self {
            uuid,
            world_name,
            position,
            data,
        }
    }

    /// Create a new [`Record`] by extending a [`PartialRecord`]
    #[inline]
    #[must_use]
    pub fn from_partial(partial: PartialRecord, data: Option<Bytes>) -> Self {
        Self {
            uuid: partial.uuid,
            world_name: partial.world_name,
            position: partial.position,
            data,
        }
    }
}
// endregion

// region: PartialRecord
/// Parts of a [`Record`] required for UUID-based lookup
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartialRecord {
    /// Unique identifier for this record
    ///
    /// **Must be globally unique**
    pub uuid: Uuid,

    /// World to store this record in
    pub world_name: String,

    /// Position of this record
    pub position: Vector3,
}

impl PartialRecord {
    /// Create a new [`PartialRecord`]
    #[inline]
    #[must_use]
    pub fn new(uuid: Uuid, world_name: String, position: Vector3) -> Self {
        Self {
            uuid,
            world_name,
            position,
        }
    }
}

impl From<Record> for PartialRecord {
    fn from(record: Record) -> Self {
        Self {
            uuid: record.uuid,
            world_name: record.world_name,
            position: record.position,
        }
    }
}
// endregion
