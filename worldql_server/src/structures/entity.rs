use bytes::Bytes;
use uuid::Uuid;

use super::{Decode, DecodeError, Encode, Vector3};
use crate::flatbuffers::EntityT;

#[derive(Debug, Default, Clone)]
pub struct Entity {
    pub uuid: Uuid,
    pub position: Vector3,
    pub world_name: String,
    pub data: Option<String>,
    pub flex: Option<Bytes>,
}

impl Encode<EntityT> for Entity {
    fn encode(self) -> EntityT {
        EntityT {
            uuid: Some(self.uuid.to_string()),
            position: Some(self.position.encode()),
            world_name: Some(self.world_name),
            data: self.data,
            flex: self.flex.map(|flex| flex.to_vec()),
        }
    }
}

impl Decode<EntityT> for Entity {
    fn decode(encoded: EntityT) -> Result<Self, DecodeError> {
        let uuid = encoded
            .uuid
            .ok_or_else(|| DecodeError::MissingRequiredField("uuid".into()))?;

        let position = encoded
            .position
            .ok_or_else(|| DecodeError::MissingRequiredField("position".into()))?;

        let world_name = encoded
            .world_name
            .ok_or_else(|| DecodeError::MissingRequiredField("world_name".into()))?;

        let entity = Entity {
            uuid: Uuid::parse_str(&uuid)?,
            position: Vector3::decode(position)?,
            world_name,
            data: encoded.data,
            flex: encoded.flex.map(Bytes::from),
        };

        Ok(entity)
    }
}
