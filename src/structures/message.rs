use flatbuffers::{FlatBufferBuilder, InvalidFlatbuffer};
use thiserror::Error;

use super::{Decode, DecodeError, Encode, Entity, Record, Vec3D};
use crate::flatbuffers::{root_as_message, MessageT};

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

        let records = match encoded.records {
            None => vec![],
            Some(records) => {
                let mut vec = vec![];
                for record in records {
                    let decoded = Record::decode(record)?;
                    vec.push(decoded);
                }

                vec
            }
        };

        let entities = match encoded.entities {
            None => vec![],
            Some(entities) => {
                let mut vec = vec![];
                for entity in entities {
                    let decoded = Entity::decode(entity)?;
                    vec.push(decoded);
                }

                vec
            }
        };

        let message = Message {
            instruction,
            sender_uuid,
            world_name,
            data: encoded.data,
            records,
            entities,
            position,
            flex: encoded.flex,
        };

        Ok(message)
    }
}

impl Message {
    pub fn serialize(self) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::with_capacity(1024);

        let encoded = self.encode();
        let offset = encoded.pack(&mut builder);

        builder.finish(offset, None);
        let buf = builder.finished_data();

        buf.to_vec()
    }

    pub fn deserialize(buf: &[u8]) -> Result<Self, DeserializeError> {
        let raw = root_as_message(buf)?;
        let message_t = raw.unpack();

        let message = Message::decode(message_t)?;
        Ok(message)
    }
}

#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error(transparent)]
    InvalidFlatbuffer(#[from] InvalidFlatbuffer),

    #[error(transparent)]
    DecodeError(#[from] DecodeError),
}
