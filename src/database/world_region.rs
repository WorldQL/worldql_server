use std::fmt::Display;

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

// define regions by their lowest possible value.
fn negative_aware_region_align(c: i64, region_size: u16) -> i64 {
    let rs: i64 = i64::from(region_size);
    if c >= 0 {
        c - (c % rs)
    } else {
        c - rs + (c.abs() % rs)
    }
}

fn negative_aware_min_bound(c: i64, table_size: i64) -> i64 {
    if c >= 0 {
        c - (c % table_size)
    } else {
        (c + (c.abs() % table_size)) - table_size
    }
}

impl WorldRegion {
    pub(super) fn new(
        world_name: &str,
        vector: &Vector3,
        region_x_size: u16,
        region_y_size: u16,
        region_z_size: u16,
    ) -> Self {
        let x = *vector.x() as i64;
        let y = *vector.y() as i64;
        let z = *vector.z() as i64;

        let x = negative_aware_region_align(x, region_x_size);
        let y = negative_aware_region_align(y, region_y_size);
        let z = negative_aware_region_align(z, region_z_size);

        Self {
            world_name: world_name.into(),
            x,
            y,
            z,
        }
    }

    #[inline]
    pub(super) fn x_bounds(&self, table_size: i64) -> (i64, i64) {
        let min_x = negative_aware_min_bound(self.x, table_size);
        let max_x = min_x + table_size;

        (min_x, max_x)
    }

    #[inline]
    pub(super) fn y_bounds(&self, table_size: i64) -> (i64, i64) {
        let min_y = negative_aware_min_bound(self.y, table_size);
        let max_y = min_y + table_size;

        (min_y, max_y)
    }

    #[inline]
    pub(super) fn z_bounds(&self, table_size: i64) -> (i64, i64) {
        let min_z = negative_aware_min_bound(self.z, table_size);
        let max_z = min_z + table_size;

        (min_z, max_z)
    }
}

impl Display for WorldRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ world = \"{}\", x = {}, y = {}, z = {} }}",
            self.world_name, self.x, self.y, self.z
        )
    }
}
// endregion

// region: DatabaseClient Shortcut
impl DatabaseClient {
    /// Shorthand function to create a new [`WorldRegion`]
    #[inline]
    pub(super) fn world_region(&self, world_name: &str, vector: &Vector3) -> WorldRegion {
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
            let region = WorldRegion::new(world, &vector, $sizes.0, $sizes.1, $sizes.2);
            println!("{:?}", region);

            assert_eq!(region.x, $expected.0);
            assert_eq!(region.y, $expected.1);
            assert_eq!(region.z, $expected.2);
        };
    }

    macro_rules! test_conversion_to_table_coordinates {
        ($input: expr, $sizes: expr, $table_sizes: expr, $expected_x: expr, $expected_y: expr, $expected_z: expr) => {
            let world = "world";
            let vector = Vector3::new($input.0, $input.1, $input.2);
            let region = WorldRegion::new(world, &vector, $sizes.0, $sizes.1, $sizes.2);

            assert_eq!(region.x_bounds($table_sizes), $expected_x);
            assert_eq!(region.y_bounds($table_sizes), $expected_y);
            assert_eq!(region.z_bounds($table_sizes), $expected_z);
        };
    }

    #[test]
    fn conversion() {
        // TODO: Add more tests
        let mc_chunk = (16, 256, 16);
        test_conversion!((0.0, 0.0, 0.0), mc_chunk, (0, 0, 0));
        test_conversion!((10.2, 84.1, 15.9), mc_chunk, (0, 0, 0));
        test_conversion!((1925.0, 54.0, 93.0), mc_chunk, (1920, 0, 80));
        test_conversion!((-45.0, 22.0, -1023.0), mc_chunk, (-48, 0, -1024));

        test_conversion_to_table_coordinates!(
            (-45.0, 22.0, -1004.0),
            mc_chunk,
            1024,
            (-1024, 0),
            (0, 1024),
            (-1024, 0)
        );

        test_conversion_to_table_coordinates!(
            // This one is subtle, the Z value is below 1024
            (-45.0, 22.0, -1015.0),
            mc_chunk,
            1024,
            (-1024, 0),
            (0, 1024),
            // But the expected table is -1024 thru -2048. Wtf?
            // It's because 1005's aligns to region z=-1024 which belongs to the next table.
            (-2048, -1024)
        );
        println!("foo");

        test_conversion_to_table_coordinates!(
            // This one is subtle, the Z value is below 1024
            (-45.0, 22.0, 1015.0),
            mc_chunk,
            1024,
            (-1024, 0),
            (0, 1024),
            // But the expected table is -1024 thru -2048. Wtf?
            // It's because 1005's aligns to region z=-1024 which belongs to the next table.
            // TODO: Fix this.
            (0, 1024)
        );
    }
}
// endregion
