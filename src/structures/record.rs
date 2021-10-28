use super::{Decode, DecodeError, Encode, Vec3D};
use crate::flatbuffers::RecordT;

#[derive(Debug, Default)]
pub struct Record {
    pub uuid: String,
    pub position: Vec3D,
    pub world_name: String,
    pub data: Option<String>,
    pub flex: Option<Vec<u8>>,
}

impl Encode<RecordT> for Record {
    fn encode(self) -> RecordT {
        RecordT {
            uuid: Some(self.uuid),
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
            .ok_or(DecodeError::MissingRequiredField("uuid".into()))?;

        let position = encoded
            .position
            .ok_or(DecodeError::MissingRequiredField("position".into()))?;

        let world_name = encoded
            .world_name
            .ok_or(DecodeError::MissingRequiredField("world_name".into()))?;

        let record = Record {
            uuid,
            position: Vec3D::decode(position)?,
            world_name,
            data: encoded.data,
            flex: encoded.flex,
        };

        Ok(record)
    }
}
