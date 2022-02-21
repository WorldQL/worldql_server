use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use serde::{Deserialize, Serialize};

/// A position in 3D space
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    /// Create a new [`Vector3`]
    ///
    /// # Examples
    /// ```
    /// use worldql_messages::common::Vector3;
    ///
    /// let pos = Vector3::new(0.0, 1.0, 1.5);
    ///
    /// assert_eq!(pos.x(), 0.0);
    /// assert_eq!(pos.y(), 1.0);
    /// assert_eq!(pos.z(), 1.5);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Create a [`Vector3`] with all components set to zero
    ///
    /// # Examples
    /// ```
    /// use worldql_messages::common::Vector3;
    ///
    /// let pos = Vector3::zero();
    ///
    /// assert_eq!(pos.x(), 0.0);
    /// assert_eq!(pos.y(), 0.0);
    /// assert_eq!(pos.z(), 0.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Returns the X component of the vector
    #[inline]
    #[must_use]
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Returns the Y component of the vector
    #[inline]
    #[must_use]
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Returns the Z component of the vector
    #[inline]
    #[must_use]
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Returns a tuple of XYZ coordinates, useful for destructuring
    #[inline]
    #[must_use]
    pub fn coords(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

// region: Conversion
// region: Array
impl From<[f64; 3]> for Vector3 {
    #[inline]
    fn from([x, y, z]: [f64; 3]) -> Self {
        Self { x, y, z }
    }
}

impl From<Vector3> for [f64; 3] {
    #[inline]
    fn from(vector: Vector3) -> Self {
        [vector.x, vector.y, vector.z]
    }
}

impl From<&Vector3> for [f64; 3] {
    #[inline]
    fn from(vector: &Vector3) -> Self {
        [vector.x, vector.y, vector.z]
    }
}
// endregion

// region: Tuple
impl From<(f64, f64, f64)> for Vector3 {
    #[inline]
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}

impl From<Vector3> for (f64, f64, f64) {
    #[inline]
    fn from(vector: Vector3) -> Self {
        (vector.x, vector.y, vector.z)
    }
}

impl From<&Vector3> for (f64, f64, f64) {
    #[inline]
    fn from(vector: &Vector3) -> Self {
        (vector.x, vector.y, vector.z)
    }
}
// endregion
// endregion

// region: (De)serialization
impl<'de> Deserialize<'de> for Vector3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let array = <[f64; 3] as Deserialize>::deserialize(deserializer)?;
        Ok(array.into())
    }
}

impl Serialize for Vector3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let array = self.into();
        <[f64; 3] as Serialize>::serialize(&array, serializer)
    }
}
// endregion

// region: Display Trait
impl Display for Vector3 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ x = {:.4}, y = {:.4}, z = {:.4} }}",
            self.x, self.y, self.z
        )
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
