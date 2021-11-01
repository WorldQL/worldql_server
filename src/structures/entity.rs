use uuid::Uuid;

use super::{Decode, DecodeError, Encode, Vec3D};
use crate::flatbuffers::EntityT;

#[derive(Debug, Default)]
pub struct Entity {
    pub uuid: Uuid,
    pub position: Vec3D,
    pub world_name: String,
    pub data: Option<String>,
    pub flex: Option<Vec<u8>>,
}

impl Encode<EntityT> for Entity {
    fn encode(self) -> EntityT {
        EntityT {
            uuid: Some(self.uuid.to_string()),
            position: Some(self.position.encode()),
            world_name: Some(self.world_name),
            data: self.data,
            flex: self.flex,
        }
    }
}

impl Decode<EntityT> for Entity {
    fn decode(encoded: EntityT) -> Result<Self, DecodeError> {
        let uuid = encoded
            .uuid
            .ok_or(DecodeError::MissingRequiredField("uuid".into()))?;

        let position = encoded
            .position
            .ok_or(DecodeError::MissingRequiredField("position".into()))?;

        let world_name = encoded
            .world_name
            .ok_or(DecodeError::MissingRequiredField("world_name".into()))?;

        let entity = Entity {
            uuid: Uuid::parse_str(&uuid)?,
            position: Vec3D::decode(position)?,
            world_name,
            data: encoded.data,
            flex: encoded.flex,
        };

        Ok(entity)
    }
}
