use super::{Entity, Record, Vec3D};

#[derive(Debug)]
pub struct Message {
    instruction: String,
    sender_uuid: String,
    world_name: String,
    data: Option<String>,
    records: Vec<Record>,
    entities: Vec<Entity>,
    position: Option<Vec3D>,
    flex: Vec<u8>,
}
