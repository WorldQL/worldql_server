use thiserror::Error;

use super::{Decode, DecodeError, Encode, Entity, Record, Vec3D};
use crate::flatbuffers::MessageT;

#[derive(Debug, Default)]
pub struct Message {
    pub instruction: String,
    pub sender_uuid: String,
    pub world_name: String,
    pub data: Option<String>,
    pub records: Vec<Record>,
    pub entities: Vec<Entity>,
    pub position: Option<Vec3D>,
    pub flex: Option<Vec<u8>>,
}

impl Encode<MessageT> for Message {
    fn encode(self) -> MessageT {
        let records = self
            .records
            .into_iter()
            .map(|r| r.encode())
            .collect::<Vec<_>>();

        let entities = self
            .entities
            .into_iter()
            .map(|e| e.encode())
            .collect::<Vec<_>>();

        MessageT {
            instruction: Some(self.instruction),
            sender_uuid: Some(self.sender_uuid),
            world_name: Some(self.world_name),
            data: self.data,
            records: Some(records),
            entities: Some(entities),
            position: self.position.map(|p| p.encode()),
            flex: self.flex,
        }
    }
}

impl Decode<MessageT> for Message {
    fn decode(encoded: MessageT) -> Result<Self, DecodeError> {
        let instruction = encoded
            .instruction
            .ok_or(DecodeError::MissingRequiredField("instruction".into()))?;

        let sender_uuid = encoded
            .sender_uuid
            .ok_or(DecodeError::MissingRequiredField("sender_uuid".into()))?;

        let world_name = encoded
            .world_name
            .ok_or(DecodeError::MissingRequiredField("world_name".into()))?;

        let position = match encoded.position {
            None => None,
            Some(pos) => Some(Vec3D::decode(pos)?),
        };

        let message = Message {
            instruction,
            sender_uuid,
            world_name,
            data: encoded.data,

            // TODO
            records: vec![],
            entities: vec![],

            position,
            flex: encoded.flex,
        };

        Ok(message)
    }
}

impl Message {
    pub fn serialize(&self) -> Vec<u8> {
        // TODO
        todo!()
    }

    pub fn deserialize(buf: &[u8]) -> Result<Self, DeserializeError> {
        // TODO
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum DeserializeError {}
