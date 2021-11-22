use derive_getters::Getters;

use super::DatabaseClient;
use crate::structures::Vector3;

// region: WorldRegion Struct
#[derive(Debug, Getters, Clone, PartialEq, Eq, Hash)]
pub(super) struct WorldRegion {
    world_name: String,
    x: i64,
    y: i64,
    z: i64,
}

impl WorldRegion {
    pub(super) fn new(
        world_name: &str,
        vector: Vector3,
        region_x_size: u16,
        region_y_size: u16,
        region_z_size: u16,
    ) -> Self {
        let x = *vector.x() as i64;
        let y = *vector.y() as i64;
        let z = *vector.z() as i64;

        let x = x - (x % i64::from(region_x_size));
        let y = y - (y % i64::from(region_y_size));
        let z = z - (z % i64::from(region_z_size));

        Self {
            world_name: world_name.into(),
            x,
            y,
            z,
        }
    }

    #[inline]
    pub(super) fn x_bounds(&self, table_size: i64) -> (i64, i64) {
        let min_x = self.x - (self.x % table_size);
        let max_x = min_x + table_size;

        (min_x, max_x)
    }

    #[inline]
    pub(super) fn y_bounds(&self, table_size: i64) -> (i64, i64) {
        let min_y = self.y - (self.y % table_size);
        let max_y = min_y + table_size;

        (min_y, max_y)
    }

    #[inline]
    pub(super) fn z_bounds(&self, table_size: i64) -> (i64, i64) {
        let min_z = self.z - (self.z % table_size);
        let max_z = min_z + table_size;

        (min_z, max_z)
    }
}
// endregion

// region: DatabaseClient Shortcut
impl DatabaseClient {
    /// Shorthand function to create a new [`WorldRegion`]
    #[inline]
    pub(super) fn world_region(&self, world_name: &str, vector: Vector3) -> WorldRegion {
        WorldRegion::new(
            world_name,
            vector,
            self.region_x_size(),
            self.region_y_size(),
            self.region_z_size(),
        )
    }
}
// endregion

// region: Tests
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_conversion {
        ($input: expr, $sizes: expr, $expected: expr) => {
            let world = "world";
            let vector = Vector3::new($input.0, $input.1, $input.2);
            let region = WorldRegion::new(world, vector, $sizes.0, $sizes.1, $sizes.2);

            assert_eq!(region.x, $expected.0);
            assert_eq!(region.y, $expected.1);
            assert_eq!(region.z, $expected.2);
        };
    }

    #[test]
    fn conversion() {
        // TODO: Add more tests
        test_conversion!((0.0, 0.0, 0.0), (16, 256, 16), (0, 0, 0));
        test_conversion!((10.2, 84.1, 15.9), (16, 256, 16), (0, 0, 0));
        test_conversion!((1925.0, 54.0, 93.0), (16, 256, 16), (1920, 0, 80));
    }
}
// endregion
