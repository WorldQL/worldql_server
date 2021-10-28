use flatbuffers::{FlatBufferBuilder, InvalidFlatbuffer};
use thiserror::Error;

use super::{Entity, Record, Vec3D};
use crate::flatbuffers::{Message as MessageFB, MessageArgs, root_as_message};

#[derive(Debug, Default)]
pub struct Message {
    instruction: String,
    sender_uuid: String,
    world_name: String,
    data: Option<String>,
    records: Vec<Record>,
    entities: Vec<Entity>,
    position: Option<Vec3D>,
    flex: Option<Vec<u8>>,
}

impl Message {
    pub fn serialize(&self) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::with_capacity(1024);

        let instruction = Some(builder.create_string(&self.instruction));
        let sender_uuid = Some(builder.create_string(&self.sender_uuid));
        let world_name = Some(builder.create_string(&self.world_name));

        let flex = match &self.flex {
            None => None,
            Some(flex) => Some(builder.create_vector(flex))
        };

        let offset = MessageFB::create(
            &mut builder,
            &MessageArgs {
                instruction,
                sender_uuid,
                world_name,

                flex,
                ..Default::default()
            },
        );

        builder.finish(offset, None);
        let bytes = builder.finished_data();

        bytes.to_vec()
    }

    pub fn deserialize(buf: &[u8]) -> Result<Self, DeserializeError> {
        let raw_message = root_as_message(buf)?;

        // Validate required fields
        let instruction = raw_message
            .instruction()
            .ok_or(DeserializeError::MissingRequiredField("instruction".into()))?
            .into();
        let sender_uuid = raw_message
            .sender_uuid()
            .ok_or(DeserializeError::MissingRequiredField("sender_uuid".into()))?
            .into();
        let world_name = raw_message
            .world_name()
            .ok_or(DeserializeError::MissingRequiredField("world_name".into()))?
            .into();

        let message = Message {
            instruction,
            sender_uuid,
            world_name,

            // TODO: Rest of the fields
            ..Default::default()
        };

        Ok(message)
    }
}

#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error("missing required field: {0}")]
    MissingRequiredField(String),

    #[error(transparent)]
    InvalidFlatbuffer(#[from] InvalidFlatbuffer),
}
