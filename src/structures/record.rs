use super::Vec3D;

#[derive(Debug, Default)]
pub struct Record {
    uuid: String,
    position: Vec3D,
    world_name: String,
    data: Option<String>,
    flex: Vec<u8>,
}
