use flatbuffers::{FlatBufferBuilder, InvalidFlatbuffer};
use thiserror::Error;
use uuid::Uuid;

use super::{Decode, DecodeError, Encode, Entity, Instruction, Record, Vector3};
use crate::flatbuffers::{root_as_message, MessageT};

#[derive(Debug, Default, Clone)]
pub struct Message {
    pub instruction: Instruction,
    pub parameter: Option<String>,
    pub sender_uuid: Uuid,
    pub world_name: Option<String>,
    pub records: Vec<Record>,
    pub entities: Vec<Entity>,
    pub position: Option<Vector3>,
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
            instruction: self.instruction.encode(),
            parameter: self.parameter,
            sender_uuid: Some(self.sender_uuid.to_string()),
            world_name: self.world_name,
            records: Some(records),
            entities: Some(entities),
            position: self.position.map(|p| p.encode()),
            flex: self.flex,
        }
    }
}

impl Decode<MessageT> for Message {
    fn decode(encoded: MessageT) -> Result<Self, DecodeError> {
        let instruction = Instruction::decode(encoded.instruction)?;

        let sender_uuid = encoded
            .sender_uuid
            .ok_or_else(|| DecodeError::MissingRequiredField("sender_uuid".into()))?;

        let position = match encoded.position {
            None => None,
            Some(pos) => Some(Vector3::decode(pos)?),
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
            parameter: encoded.parameter,
            sender_uuid: Uuid::parse_str(&sender_uuid)?,
            world_name: encoded.world_name,
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
