use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use derive_getters::Getters;

use super::{Decode, DecodeError, Encode};
use crate::flatbuffers::Vec3dT;
use crate::subscriptions::CubeArea;

#[derive(Debug, Default, Getters, Clone, Copy, PartialEq)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

// region: Conversion Traits
impl From<CubeArea> for Vector3 {
    fn from(area: CubeArea) -> Self {
        let x = *area.x() as f64;
        let y = *area.y() as f64;
        let z = *area.z() as f64;

        Self::new(x, y, z)
    }
}
// endregion

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

    #[inline]
    fn add(self, rhs: Vector3) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Self::new(x, y, z)
    }
}

impl AddAssign for Vector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Vector3) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Self::new(x, y, z)
    }
}

impl SubAssign for Vector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Div for Vector3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Vector3) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        let z = self.z / rhs.z;
        Self::new(x, y, z)
    }
}

impl DivAssign for Vector3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        Self::new(x, y, z)
    }
}

impl DivAssign<f64> for Vector3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Mul for Vector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Vector3) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        Self::new(x, y, z)
    }
}

impl MulAssign for Vector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Self::new(x, y, z)
    }
}

impl MulAssign<f64> for Vector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Vector3) -> Self::Output {
        let x = self * rhs.x;
        let y = self * rhs.y;
        let z = self * rhs.z;
        Self::Output::new(x, y, z)
    }
}

impl Neg for Vector3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        Self::new(x, y, z)
    }
}
// endregion
