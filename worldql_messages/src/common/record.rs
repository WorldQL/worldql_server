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
    ///
    /// # Examples
    /// ```
    /// use uuid::Uuid;
    /// use worldql_messages::common::{Record, Vector3};
    ///
    /// let record = Record::new(Uuid::nil(), "world".to_owned(), Vector3::zero(), None);
    ///
    /// assert_eq!(record.uuid, Uuid::nil());
    /// assert_eq!(record.world_name, "world");
    /// assert_eq!(record.position.x(), 0.0);
    /// assert_eq!(record.position.y(), 0.0);
    /// assert_eq!(record.position.z(), 0.0);
    /// assert_eq!(record.data, None);
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// use uuid::Uuid;
    /// use worldql_messages::common::{Record, PartialRecord, Vector3};
    ///
    /// let partial = PartialRecord::new(Uuid::nil(), "world".to_owned(), Vector3::zero());
    /// let record = Record::from_partial(partial, None);
    ///
    /// assert_eq!(record.uuid, Uuid::nil());
    /// assert_eq!(record.world_name, "world");
    /// assert_eq!(record.position.x(), 0.0);
    /// assert_eq!(record.position.y(), 0.0);
    /// assert_eq!(record.position.z(), 0.0);
    /// assert_eq!(record.data, None);
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// use uuid::Uuid;
    /// use worldql_messages::common::{PartialRecord, Vector3};
    ///
    /// let record = PartialRecord::new(Uuid::nil(), "world".to_owned(), Vector3::zero());
    ///
    /// assert_eq!(record.uuid, Uuid::nil());
    /// assert_eq!(record.world_name, "world");
    /// assert_eq!(record.position.x(), 0.0);
    /// assert_eq!(record.position.y(), 0.0);
    /// assert_eq!(record.position.z(), 0.0);
    /// ```
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
