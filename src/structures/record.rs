use uuid::Uuid;

use super::{Decode, DecodeError, Encode, Vector3};
use crate::flatbuffers::RecordT;

#[derive(Debug, Default)]
pub struct Record {
    pub uuid: Uuid,
    pub position: Vector3,
    pub world_name: String,
    pub data: Option<String>,
    pub flex: Option<Vec<u8>>,
}

impl Encode<RecordT> for Record {
    fn encode(self) -> RecordT {
        RecordT {
            uuid: Some(self.uuid.to_string()),
            position: Some(self.position.encode()),
            world_name: Some(self.world_name),
            data: self.data,
            flex: self.flex,
        }
    }
}

impl Decode<RecordT> for Record {
    fn decode(encoded: RecordT) -> Result<Self, DecodeError> {
        let uuid = encoded
            .uuid
            .ok_or_else(|| DecodeError::MissingRequiredField("uuid".into()))?;

        let position = encoded
            .position
            .ok_or_else(|| DecodeError::MissingRequiredField("position".into()))?;

        let world_name = encoded
            .world_name
            .ok_or_else(|| DecodeError::MissingRequiredField("world_name".into()))?;

        let record = Record {
            uuid: Uuid::parse_str(&uuid)?,
            position: Vector3::decode(position)?,
            world_name,
            data: encoded.data,
            flex: encoded.flex,
        };

        Ok(record)
    }
}
