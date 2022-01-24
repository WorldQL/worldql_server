use bytes::Bytes;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Vector3;

// region: Record
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Record {
    pub uuid: Uuid,
    pub world_name: String,
    pub position: Vector3,
    pub data: Option<Bytes>,
}

impl Record {
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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartialRecord {
    pub uuid: Uuid,
    pub world_name: String,
    pub position: Vector3,
}

impl PartialRecord {
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
