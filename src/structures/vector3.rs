use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::{Decode, DecodeError, Encode};
use crate::flatbuffers::Vec3dT;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    fn coord_clamp(coord: f64, size_f: f64) -> f64 {
        // Either 0 or 10
        let clamped = if coord % size_f == 0.0 {
            0.0
        } else {
            size_f
        };

        let floored = crate::utils::floor_by_multiple(coord, size_f);
        let combined = floored + clamped;

        combined
    }

    pub fn cube_clamp(self, size: u16) -> Self {
        let size_f = size as f64;

        let x = Self::coord_clamp(self.x, size_f);
        let y = Self::coord_clamp(self.y, size_f);
        let z = Self::coord_clamp(self.z, size_f);

        Self::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_coord_clamp {
        ($input: expr, $expected: expr, $clamp: expr) => {
            let input = Vector3::coord_clamp($input, $clamp);
            assert_eq!(input, $expected)
        };
    }

    #[test]
    fn coord_clamp() {
        // Unit Case
        test_coord_clamp!(0.0, 0.0, 10.0);

        // Positive Cases
        test_coord_clamp!(0.1, 10.0, 10.0);
        test_coord_clamp!(5.0, 10.0, 10.0);
        test_coord_clamp!(9.999, 10.0, 10.0);
        test_coord_clamp!(10.0, 10.0, 10.0);
        test_coord_clamp!(10.1, 20.0, 10.0);
    }

    macro_rules! test_cube_clamp {
        ($input: expr, $expected: expr, $clamp: expr) => {
            let input = Vector3::new($input.0, $input.1, $input.2);
            let expected = Vector3::new($expected.0, $expected.1, $expected.2);

            assert_eq!(expected, input.cube_clamp($clamp));
        };
    }

    #[test]
    fn cube_clamp() {
        // Unit case
        test_cube_clamp!((0.0, 0.0, 0.0), (0.0, 0.0, 0.0), 10);

        // Positive Cases
        test_cube_clamp!((0.1, 0.3, 2.5), (10.0, 10.0, 10.0), 10);
        test_cube_clamp!((3.0, 4.0, 5.0), (10.0, 10.0, 10.0), 10);
        test_cube_clamp!((18.0, 12.0, 16.0), (20.0, 20.0, 20.0), 10);

        // Negative Cases
        test_cube_clamp!((-3.0, -8.0, -1.3), (0.0, 0.0, 0.0), 10);
        test_cube_clamp!((-6.0, -0.3, -9.9), (0.0, 0.0, 0.0), 10);
        test_cube_clamp!((-12.0, -19.9, -13.5), (-10.0, -10.0, -10.0), 10);
    }
}

// region: Codec Traits
impl Encode<Vec3dT> for Vector3 {
    fn encode(self) -> Vec3dT {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        Vec3dT { x, y, z }
    }
}

impl Decode<Vec3dT> for Vector3 {
    fn decode(encoded: Vec3dT) -> Result<Self, DecodeError> {
        let x = encoded.x;
        let y = encoded.y;
        let z = encoded.z;

        Ok(Self { x, y, z })
    }
}
// endregion

// region: Math Traits
impl Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Vector3) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Self::new(x, y, z)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Vector3) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Self::new(x, y, z)
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Div for Vector3 {
    type Output = Self;
    fn div(self, rhs: Vector3) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        let z = self.z / rhs.z;
        Self::new(x, y, z)
    }
}

impl DivAssign for Vector3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        Self::new(x, y, z)
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Mul for Vector3 {
    type Output = Self;
    fn mul(self, rhs: Vector3) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        Self::new(x, y, z)
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Self::new(x, y, z)
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        let x = self * rhs.x;
        let y = self * rhs.y;
        let z = self * rhs.z;
        Self::Output::new(x, y, z)
    }
}

impl Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        Self::new(x, y, z)
    }
}
// endregion
