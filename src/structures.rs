#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
pub struct Record {
    uuid: String,
    position: Vec3D,
    world_name: String,
    data: Option<String>,
    flex: Vec<u8>,
}

#[derive(Debug)]
pub struct Entity {
    uuid: String,
    position: Vec3D,
    world_name: String,
    data: Option<String>,
    flex: Vec<u8>,
}

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
