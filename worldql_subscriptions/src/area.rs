use std::fmt::Display;

use crate::clamp::coord_clamp;

// region: Area
/// 3D representation of an area
///
/// Not usually constructed manually
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Area {
    x: i64,
    y: i64,
    z: i64,
}

impl Area {
    /// Create a new [`Area`]
    ///
    /// # Examples
    /// ```
    /// use worldql_subscriptions::Area;
    ///
    /// let area = Area::new(0, 1, -1);
    ///
    /// assert_eq!(area.x(), 0);
    /// assert_eq!(area.y(), 1);
    /// assert_eq!(area.z(), -1);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    /// Create a new [`Area`] by clamping float coordinates
    ///
    /// # Examples
    /// ```
    /// use worldql_subscriptions::Area;
    ///
    /// let zero_area = Area::new_clamped(0.0, 0.0, 0.0, 16);
    /// assert_eq!(zero_area.x(), 16);
    /// assert_eq!(zero_area.y(), 16);
    /// assert_eq!(zero_area.z(), 16);
    /// ```
    #[inline]
    #[must_use]
    pub fn new_clamped(x: f64, y: f64, z: f64, size: u16) -> Self {
        let x = coord_clamp(x, size);
        let y = coord_clamp(y, size);
        let z = coord_clamp(z, size);

        Self { x, y, z }
    }

    /// Returns the maximum X coordinate for the area
    #[inline]
    #[must_use]
    pub fn x(&self) -> i64 {
        self.x
    }

    /// Returns the maximum Y coordinate for the area
    #[inline]
    #[must_use]
    pub fn y(&self) -> i64 {
        self.y
    }

    /// Returns the maximum Z coordinate for the area
    #[inline]
    #[must_use]
    pub fn z(&self) -> i64 {
        self.z
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ x = {}, y = {}, z = {} }}", self.x, self.y, self.z)
    }
}
// endregion

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_new_clamped {
        ($input: expr, $expected: expr, $clamp: expr) => {
            let expected = Area::new($expected.0, $expected.1, $expected.2);
            let actual = Area::new_clamped($input.0, $input.1, $input.2, $clamp);

            assert_eq!(actual, expected);
        };
    }

    #[test]
    fn from_vector3() {
        // Unit case
        test_new_clamped!((0.0, 0.0, 0.0), (10, 10, 10), 10);

        // Positive Cases
        test_new_clamped!((0.1, 0.3, 2.5), (10, 10, 10), 10);
        test_new_clamped!((3.0, 4.0, 5.0), (10, 10, 10), 10);
        test_new_clamped!((9.1, 9.9, 9.9), (10, 10, 10), 10);
        test_new_clamped!((18.0, 12.5, 16.7), (20, 20, 20), 10);

        // Negative Cases
        test_new_clamped!((-3.0, -8.0, -1.3), (-10, -10, -10), 10);
        test_new_clamped!((-6.0, -0.3, -9.9), (-10, -10, -10), 10);
        test_new_clamped!((-12.0, -19.9, -13.5), (-20, -20, -20), 10);

        // Mixed Cases
        test_new_clamped!((25.0, -13.2, 0.0), (30, -20, 10), 10);
        test_new_clamped!((25.0, -13.2, -0.1), (30, -20, -10), 10);
    }
}
