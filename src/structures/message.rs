use std::fmt::Display;

use bytes::Bytes;
use flatbuffers::{FlatBufferBuilder, InvalidFlatbuffer};
use thiserror::Error;
use uuid::Uuid;
use tracing::{debug, info, warn};

use super::{Decode, DecodeError, Encode, Entity, Instruction, Record, Vector3};
use crate::flatbuffers::{root_as_message, MessageT};

#[derive(Debug, Default, Clone)]
pub struct Message {
    pub instruction: Instruction,
    pub parameter: Option<String>,
    pub sender_uuid: Uuid,
    pub world_name: String,
    pub records: Vec<Record>,
    pub entities: Vec<Entity>,
    pub position: Option<Vector3>,
    pub flex: Option<Bytes>,
}

// region: Codec Traits
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
            world_name: Some(self.world_name),
            records: Some(records),
            entities: Some(entities),
            position: self.position.map(|p| p.encode()),
            flex: self.flex.map(|flex| flex.to_vec()),
        }
    }
}

impl Decode<MessageT> for Message {
    fn decode(encoded: MessageT) -> Result<Self, DecodeError> {
        let instruction = Instruction::decode(encoded.instruction)?;

        let world_name = encoded
            .world_name
            .ok_or_else(|| DecodeError::MissingRequiredField("world_name".into()))?;

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
            world_name,
            records,
            entities,
            position,
            flex: encoded.flex.map(Bytes::from),
        };

        Ok(message)
    }
}
// endregion

// region: (De)serialization
impl Message {
    pub fn serialize(self) -> Bytes {
        let mut builder = FlatBufferBuilder::with_capacity(1024);

        let encoded = self.encode();
        let offset = encoded.pack(&mut builder);

        builder.finish(offset, None);
        let buf = builder.finished_data();

        Bytes::from(buf.to_vec())
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
// endregion

// region: Display Trait
macro_rules! write_optional {
    ($f: expr, $self: expr) => {{
        if let Some(parameter) = &$self.parameter {
            write!($f, ", parameter = \"{}\"", parameter)?;
        }

        if let Some(flex) = &$self.flex {
            write!($f, ", flex = [u8; {}]", flex.len())?;
        }
    }};
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.instruction {
            Instruction::Heartbeat | Instruction::Handshake => {
                write!(
                    f,
                    "{} = {{ sender = \"{}\"",
                    self.instruction, self.sender_uuid
                )?;

                write_optional!(f, self);
                write!(f, " }}")
            }

            Instruction::PeerConnect | Instruction::PeerDisconnect => write!(
                f,
                "{} = {{ peer = \"{}\" }}",
                self.instruction,
                self.parameter.as_ref().unwrap()
            ),

            Instruction::AreaSubscribe | Instruction::AreaUnsubscribe => write!(
                f,
                "{} = {{ sender = \"{}\", world = \"{}\", area = {} }}",
                self.instruction,
                self.sender_uuid,
                self.world_name,
                self.position.as_ref().unwrap()
            ),

            Instruction::GlobalMessage => {
                write!(
                    f,
                    "{} = {{ sender = \"{}\", world = \"{}\"",
                    self.instruction, self.sender_uuid, self.world_name
                )?;

                write_optional!(f, self);
                write!(f, " }}")
            }

            Instruction::LocalMessage => {
                write!(
                    f,
                    "{} = {{ sender = \"{}\", world = \"{}\", position = {}",
                    self.instruction,
                    self.sender_uuid,
                    self.world_name,
                    self.position.as_ref().unwrap()
                )?;

                write_optional!(f, self);
                write!(f, " }}")
            }

            Instruction::RecordCreate => todo!(),
            Instruction::RecordRead => todo!(),
            Instruction::RecordUpdate => todo!(),
            Instruction::RecordDelete => todo!(),
            Instruction::RecordReply => todo!(),

            Instruction::Unknown => write!(
                f,
                "{} = {{ sender = \"{}\" }}",
                self.instruction, self.sender_uuid
            ),
        }
    }
}
// endregion
