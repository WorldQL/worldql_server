use bytes::Bytes;
use tokio_postgres::Row;
use uuid::Uuid;

use super::{Decode, DecodeError, Encode, Vector3};
use crate::flatbuffers::RecordT;

#[derive(Debug, Default, Clone)]
pub struct Record {
    pub uuid: Uuid,
    pub position: Option<Vector3>,
    pub world_name: String,
    pub data: Option<String>,
    pub flex: Option<Bytes>,
}

impl Encode<RecordT> for Record {
    fn encode(self) -> RecordT {
        RecordT {
            uuid: Some(self.uuid.to_string()),
            position: self.position.map(|p| p.encode()),
            world_name: Some(self.world_name),
            data: self.data,
            flex: self.flex.map(|flex| flex.to_vec()),
        }
    }
}

impl Decode<RecordT> for Record {
    fn decode(encoded: RecordT) -> Result<Self, DecodeError> {
        let uuid = encoded
            .uuid
            .ok_or_else(|| DecodeError::MissingRequiredField("uuid".into()))?;

        let position = match encoded.position {
            None => None,
            Some(pos) => Some(Vector3::decode(pos)?),
        };

        let world_name = encoded
            .world_name
            .ok_or_else(|| DecodeError::MissingRequiredField("world_name".into()))?;

        let record = Record {
            uuid: Uuid::parse_str(&uuid)?,
            position,
            world_name,
            data: encoded.data,
            flex: encoded.flex.map(Bytes::from),
        };

        Ok(record)
    }
}

impl Record {
    pub fn from_postgres_row(row: Row, world_name: &str) -> Self {
        let x: f64 = row.get("x");
        let y: f64 = row.get("y");
        let z: f64 = row.get("z");
        let flex: Option<Vec<u8>> = row.get("flex");

        Self {
            uuid: row.get("uuid"),
            position: Some(Vector3::new(x, y, z)),
            world_name: world_name.to_string(),
            data: row.get("data"),
            flex: flex.map(Bytes::from),
        }
    }
}
