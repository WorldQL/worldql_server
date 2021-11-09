use super::CubeArea;
use crate::structures::Vector3;

pub trait ToCubeArea {
    fn to_cube_area(self, size: u16) -> CubeArea;
}

impl ToCubeArea for CubeArea {
    fn to_cube_area(self, _: u16) -> CubeArea {
        self
    }
}

impl ToCubeArea for Vector3 {
    fn to_cube_area(self, size: u16) -> CubeArea {
        CubeArea::from_vector3(self, size)
    }
}
