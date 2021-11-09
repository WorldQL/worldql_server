use crate::structures::Vector3;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CubeArea {
    x: i64,
    y: i64,
    z: i64,
}

impl CubeArea {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
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

    fn coord_clamp(coord: f64, size: u32) -> i64 {
        let size_i = size as i64;
        let size_f = size as f64;

        let clamped = if coord % size_f == 0.0 { 0 } else { size_i };

        let floored = crate::utils::floor_by_multiple(coord as u32, size) as i64;
        let combined = floored + clamped;

        combined
    }

    pub fn from_vector3(vec: Vector3, size: u16) -> Self {
        todo!()
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
    fn coord_clamp() {
        // Unit Case
        test_coord_clamp!((0.0, 10), 0);

        // Positive Cases
        test_coord_clamp!((0.1, 10), 10);
        test_coord_clamp!((5.0, 10), 10);
        test_coord_clamp!((9.99999, 10), 10);
        test_coord_clamp!((10.0, 10), 10);
        test_coord_clamp!((10.1, 10), 20);

        // Negative Cases
        test_coord_clamp!((-0.1, 0), 0);
        test_coord_clamp!((-5.0, 0), 0);
        test_coord_clamp!((-9.99999, 0), 0);
        test_coord_clamp!((-10.0, 10), -10);
    }

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
