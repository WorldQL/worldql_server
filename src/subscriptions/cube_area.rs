use std::fmt::Display;

use derive_getters::Getters;

use crate::structures::Vector3;

// region: CubeArea
#[derive(Debug, Default, Getters, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CubeArea {
    x: i64,
    y: i64,
    z: i64,
}

impl CubeArea {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    /// Clamp to largest absolute coordinate value.
    ///
    /// This allows us to disambiguate positive and negative areas.
    fn coord_clamp(coord: f64, size: u16) -> i64 {
        let abs_coord = coord.abs();
        let result_multiplier = match coord < 0.0 {
            true => -1,
            false => 1,
        };

        let size_i = size as i64;
        let size_f = size as f64;

        if abs_coord % size_f == 0.0 && coord != 0.0 {
            return coord as i64;
        }

        let rounded = crate::utils::round_by_multiple(abs_coord, size as f64);
        let result = match rounded > coord {
            true => rounded as i64,
            false => (rounded as i64) + size_i,
        };

        result * result_multiplier
    }

    /// Convert a [`Vector3`] to a [`CubeArea`]
    ///
    /// Vector3 also implements [`ToCubeArea`] which implicitly calls this function.
    #[inline]
    pub fn from_vector3(vec: Vector3, size: u16) -> Self {
        let x = Self::coord_clamp(*vec.x(), size);
        let y = Self::coord_clamp(*vec.y(), size);
        let z = Self::coord_clamp(*vec.z(), size);

        Self::new(x, y, z)
    }
}
// endregion

// region: ToCubeArea Trait
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
// endregion

// region: Display Trait
impl Display for CubeArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ x = {}, y = {}, z = {} }}", self.x, self.y, self.z)
    }
}
// endregion

// region: Tests
#[cfg(test)]
mod tests {
    use super::*;

    // region: coord_clamp()
    macro_rules! test_coord_clamp {
        ($input: expr, $expected: expr) => {
            let (input, clamp) = $input;
            let actual = CubeArea::coord_clamp(input, clamp);
            assert_eq!(actual, $expected)
        };
    }

    #[test]
    fn coord_clamp_10() {
        // Unit Case
        test_coord_clamp!((0.0, 10), 10);

        // Positive Cases
        test_coord_clamp!((0.1, 10), 10);
        test_coord_clamp!((5.0, 10), 10);
        test_coord_clamp!((9.99999, 10), 10);
        test_coord_clamp!((10.0, 10), 10);
        test_coord_clamp!((10.1, 10), 20);

        // Negative Cases
        test_coord_clamp!((-0.1, 10), -10);
        test_coord_clamp!((-5.0, 10), -10);
        test_coord_clamp!((-9.99999, 10), -10);
        test_coord_clamp!((-10.0, 10), -10);
        test_coord_clamp!((-10.1, 10), -20);
        test_coord_clamp!((-20.0, 10), -20);
    }

    #[test]
    fn coord_clamp_8() {
        // Unit Case
        test_coord_clamp!((0.0, 8), 8);

        // Positive Cases
        test_coord_clamp!((0.1, 8), 8);
        test_coord_clamp!((5.0, 8), 8);
        test_coord_clamp!((9.99999, 8), 16);
        test_coord_clamp!((10.0, 8), 16);
        test_coord_clamp!((10.1, 8), 16);

        // Negative Cases
        test_coord_clamp!((-0.1, 8), -8);
        test_coord_clamp!((-5.0, 8), -8);
        test_coord_clamp!((-9.99999, 8), -16);
        test_coord_clamp!((-10.0, 8), -16);
        test_coord_clamp!((-10.1, 8), -16);
        test_coord_clamp!((-20.0, 8), -24);
    }
    // endregion

    // region: from_vector3()
    macro_rules! test_from_vector3 {
        ($input: expr, $expected: expr, $clamp: expr) => {
            let input = Vector3::new($input.0, $input.1, $input.2);
            let expected = CubeArea::new($expected.0, $expected.1, $expected.2);

            let actual = CubeArea::from_vector3(input, $clamp);
            assert_eq!(actual, expected);
        };
    }

    #[test]
    fn from_vector3() {
        // Unit case
        test_from_vector3!((0.0, 0.0, 0.0), (10, 10, 10), 10);

        // Positive Cases
        test_from_vector3!((0.1, 0.3, 2.5), (10, 10, 10), 10);
        test_from_vector3!((3.0, 4.0, 5.0), (10, 10, 10), 10);
        test_from_vector3!((9.1, 9.9, 9.9), (10, 10, 10), 10);
        test_from_vector3!((18.0, 12.5, 16.7), (20, 20, 20), 10);

        // Negative Cases
        test_from_vector3!((-3.0, -8.0, -1.3), (-10, -10, -10), 10);
        test_from_vector3!((-6.0, -0.3, -9.9), (-10, -10, -10), 10);
        test_from_vector3!((-12.0, -19.9, -13.5), (-20, -20, -20), 10);

        // Mixed Cases
        test_from_vector3!((25.0, -13.2, 0.0), (30, -20, 10), 10);
        test_from_vector3!((25.0, -13.2, -0.1), (30, -20, -10), 10);
    }
    // endregion
}
// endregion
