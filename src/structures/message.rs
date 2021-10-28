use flatbuffers::FlatBufferBuilder;

use super::{Entity, Record, Vec3D};
use crate::flatbuffers::{Message as MessageFB, MessageArgs};

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
}
