use crate::structures::Vector3;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CubeArea {
    x: i64,
    y: i64,
    z: i64,
    size: u16,
}

impl CubeArea {
    pub fn new(x: i64, y: i64, z: i64, size: u16) -> Self {
        Self { x, y, z, size }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }

    pub fn z(&self) -> i64 {
        self.z
    }

    pub fn size(&self) -> u16 {
        self.size
    }

    fn coord_clamp(coord: f64, size: u16) -> i64 {
        let size_i = size as i64;
        let size_f = size as f64;

        if coord % size_f == 0.0 {
            return coord as i64;
        }

        let rounded = crate::utils::round_by_multiple(coord, size as f64);
        match rounded > coord {
            true => rounded as i64,
            false => (rounded as i64) + size_i,
        }
    }

    pub fn from_vector3(vec: Vector3, size: u16) -> Self {
        let x = Self::coord_clamp(vec.x, size);
        let y = Self::coord_clamp(vec.y, size);
        let z = Self::coord_clamp(vec.z, size);

        Self::new(x, y, z, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        test_coord_clamp!((0.0, 10), 0);

        // Positive Cases
        test_coord_clamp!((0.1, 10), 10);
        test_coord_clamp!((5.0, 10), 10);
        test_coord_clamp!((9.99999, 10), 10);
        test_coord_clamp!((10.0, 10), 10);
        test_coord_clamp!((10.1, 10), 20);

        // Negative Cases
        test_coord_clamp!((-0.1, 10), 0);
        test_coord_clamp!((-5.0, 10), 0);
        test_coord_clamp!((-9.99999, 10), 0);
        test_coord_clamp!((-10.0, 10), -10);
        test_coord_clamp!((-10.1, 10), -10);
        test_coord_clamp!((-20.0, 10), -20);
    }

    #[test]
    fn coord_clamp_8() {
        // Unit Case
        test_coord_clamp!((0.0, 8), 0);

        // Positive Cases
        test_coord_clamp!((0.1, 8), 8);
        test_coord_clamp!((5.0, 8), 8);
        test_coord_clamp!((9.99999, 8), 16);
        test_coord_clamp!((10.0, 8), 16);
        test_coord_clamp!((10.1, 8), 16);

        // Negative Cases
        test_coord_clamp!((-0.1, 8), 0);
        test_coord_clamp!((-5.0, 8), 0);
        test_coord_clamp!((-9.99999, 8), -8);
        test_coord_clamp!((-10.0, 8), -8);
        test_coord_clamp!((-10.1, 8), -8);
        test_coord_clamp!((-20.0, 8), -16);
    }

    macro_rules! test_from_vector3 {
        ($input: expr, $expected: expr, $clamp: expr) => {
            let input = Vector3::new($input.0, $input.1, $input.2);
            let expected = CubeArea::new($expected.0, $expected.1, $expected.2, $clamp);

            let actual = CubeArea::from_vector3(input, $clamp);
            assert_eq!(actual, expected);
        };
    }

    #[test]
    fn from_vector3() {
        // Unit case
        test_from_vector3!((0.0, 0.0, 0.0), (0, 0, 0), 10);

        // Positive Cases
        test_from_vector3!((0.1, 0.3, 2.5), (10, 10, 10), 10);
        test_from_vector3!((3.0, 4.0, 5.0), (10, 10, 10), 10);
        test_from_vector3!((9.1, 9.9, 9.9), (10, 10, 10), 10);
        test_from_vector3!((18.0, 12.5, 16.7), (20, 20, 20), 10);

        // TODO: Negative Cases
        // test_cube_clamp!((-3.0, -8.0, -1.3), (0.0, 0.0, 0.0), 10);
        // test_cube_clamp!((-6.0, -0.3, -9.9), (0.0, 0.0, 0.0), 10);
        // test_cube_clamp!((-12.0, -19.9, -13.5), (-10.0, -10.0, -10.0), 10);
    }
}
