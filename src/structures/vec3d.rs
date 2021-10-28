use crate::flatbuffers::{Vec3d as Vec3DFB};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3D {
    pub fn to_flexbuffer(&self) -> Vec3DFB {
        Vec3DFB::new(self.x, self.y, self.z)
    }
}
