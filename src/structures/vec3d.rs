use super::{Decode, DecodeError, Encode};
use crate::flatbuffers::Vec3dT;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Encode<Vec3dT> for Vec3D {
    fn encode(self) -> Vec3dT {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        Vec3dT { x, y, z }
    }
}

impl Decode<Vec3dT> for Vec3D {
    fn decode(encoded: Vec3dT) -> Result<Self, DecodeError> {
        let x = encoded.x;
        let y = encoded.y;
        let z = encoded.z;

        Ok(Self { x, y, z })
    }
}
