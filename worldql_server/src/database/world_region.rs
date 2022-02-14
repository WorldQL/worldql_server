use std::fmt::Display;

use derive_getters::Getters;
use worldql_messages::common::Vector3;

use super::DatabaseClient;

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
        vector: &Vector3,
        region_x_size: u16,
        region_y_size: u16,
        region_z_size: u16,
    ) -> Self {
        let x = clamp_region_coord(vector.x(), region_x_size);
        let y = clamp_region_coord(vector.y(), region_y_size);
        let z = clamp_region_coord(vector.z(), region_z_size);

        Self {
            world_name: world_name.into(),
            x,
            y,
            z,
        }
    }

    #[inline]
    pub(super) fn x_bounds(&self, table_size: i64) -> (i64, i64) {
        let min_x = clamp_table_size(self.x, table_size);
        let max_x = min_x + table_size;

        (min_x, max_x)
    }

    #[inline]
    pub(super) fn y_bounds(&self, table_size: i64) -> (i64, i64) {
        let min_y = clamp_table_size(self.y, table_size);
        let max_y = min_y + table_size;

        (min_y, max_y)
    }

    #[inline]
    pub(super) fn z_bounds(&self, table_size: i64) -> (i64, i64) {
        let min_z = clamp_table_size(self.z, table_size);
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

// region: Coordinate Clamp Functions
/// Define region coords by their lowest possible value.
///
/// For negatives, they should still round down.
fn clamp_region_coord(c: f64, region_size: u16) -> i64 {
    // Unit case, 0 always return 0
    if c == 0.0 || c == -0.0 {
        return 0;
    }

    if c >= 0.0 {
        let region_size = i64::from(region_size);
        let c = c as i64;

        c - (c % region_size)
    } else {
        let new_c = (-c) + f64::from(region_size);
        let result = clamp_region_coord(new_c, region_size);

        -result
    }
}

fn clamp_table_size(c: i64, table_size: i64) -> i64 {
    // On a table border, return
    if c % table_size == 0 {
        return c;
    }

    if c >= 0 {
        let region_size = table_size;
        let c = c as i64;

        c - (c % region_size)
    } else {
        let new_c = (-c) + table_size;
        let result = clamp_table_size(new_c, table_size);

        -result
    }
}
// endregion

// region: Tests
#[cfg(test)]
mod tests {
    use super::*;

    // region: clamp_region_coord
    macro_rules! test_clamp_region_coord {
        ($input: expr, $region_size: expr, $expected: expr) => {
            let output = super::clamp_region_coord($input, $region_size);
            assert_eq!(output, $expected);
        };
    }

    #[test]
    fn clamp_region_coord() {
        // Unit case
        test_clamp_region_coord!(0.0, 16, 0);

        // Positive
        test_clamp_region_coord!(0.1, 16, 0);
        test_clamp_region_coord!(15.0, 16, 0);
        test_clamp_region_coord!(16.0, 16, 16);
        test_clamp_region_coord!(31.9, 16, 16);
        test_clamp_region_coord!(32.0, 16, 32);
        test_clamp_region_coord!(0.0, 256, 0);
        test_clamp_region_coord!(0.1, 256, 0);
        test_clamp_region_coord!(128.0, 256, 0);
        test_clamp_region_coord!(255.9, 256, 0);
        test_clamp_region_coord!(256.0, 256, 256);
        test_clamp_region_coord!(511.9, 256, 256);
        test_clamp_region_coord!(512.0, 256, 512);

        // Negative
        test_clamp_region_coord!(-0.1, 16, -16);
        test_clamp_region_coord!(-1.0, 16, -16);
        test_clamp_region_coord!(-15.0, 16, -16);
        test_clamp_region_coord!(-16.0, 16, -32);
        test_clamp_region_coord!(-31.9, 16, -32);
        test_clamp_region_coord!(-32.0, 16, -48);
        test_clamp_region_coord!(-32.1, 16, -48);
        test_clamp_region_coord!(-1.0, 256, -256);
        test_clamp_region_coord!(-128.0, 256, -256);
        test_clamp_region_coord!(-255.9, 256, -256);
        test_clamp_region_coord!(-256.0, 256, -512);
    }
    // endregion

    // region: clamp_table_size
    macro_rules! test_clamp_table_size {
        ($input: expr, $table_size: expr, $expected: expr) => {
            let output = super::clamp_table_size($input, $table_size);
            assert_eq!(output, $expected);
        };
    }

    #[test]
    fn clamp_table_size() {
        // Unit case
        test_clamp_table_size!(0, 1024, 0);

        // Positive
        test_clamp_table_size!(1, 1024, 0);
        test_clamp_table_size!(256, 1024, 0);
        test_clamp_table_size!(1024, 1024, 1024);
        test_clamp_table_size!(1800, 1024, 1024);
        test_clamp_table_size!(2047, 1024, 1024);
        test_clamp_table_size!(2048, 1024, 2048);

        // Negative
        test_clamp_table_size!(-1, 1024, -1024);
        test_clamp_table_size!(-45, 1024, -1024);
        test_clamp_table_size!(-687, 1024, -1024);
        test_clamp_table_size!(-1023, 1024, -1024);
        test_clamp_table_size!(-1024, 1024, -1024);
        test_clamp_table_size!(-1025, 1024, -2048);
    }
    // endregion

    // region: conversion
    macro_rules! test_conversion {
        ($input: expr, $sizes: expr, $expected: expr) => {
            let vector = Vector3::new($input.0, $input.1, $input.2);
            let region = WorldRegion::new("world", &vector, $sizes.0, $sizes.1, $sizes.2);

            assert_eq!(region.x, $expected.0);
            assert_eq!(region.y, $expected.1);
            assert_eq!(region.z, $expected.2);
        };
    }

    #[test]
    fn conversion() {
        let mc_chunk = (16, 256, 16);

        // Positive
        test_conversion!((0.0, 0.0, 0.0), mc_chunk, (0, 0, 0));
        test_conversion!((10.2, 84.1, 15.9), mc_chunk, (0, 0, 0));
        test_conversion!((10.2, 486.5, 15.9), mc_chunk, (0, 256, 0));
        test_conversion!((1925.0, 54.0, 93.0), mc_chunk, (1920, 0, 80));

        // Negative
        test_conversion!((-0.01, -0.01, -0.01), mc_chunk, (-16, -256, -16));
        test_conversion!((-15.9, -255.9, -15.9), mc_chunk, (-16, -256, -16));
        test_conversion!((-50.0, -8.4, -17.6), mc_chunk, (-64, -256, -32));
        test_conversion!((-1925.0, -478.3, -85.6), mc_chunk, (-1936, -512, -96));

        // Mixed
        test_conversion!((-45.0, 22.0, -1023.0), mc_chunk, (-48, 0, -1024));
    }
    // endregion

    // region: table_bounds
    macro_rules! test_table_bounds {
        ($input: expr, $sizes: expr, $table_sizes: expr, $expected_x: expr, $expected_y: expr, $expected_z: expr) => {
            let vector = Vector3::new($input.0, $input.1, $input.2);
            let region = WorldRegion::new("world", &vector, $sizes.0, $sizes.1, $sizes.2);

            assert_eq!(region.x_bounds($table_sizes), $expected_x);
            assert_eq!(region.y_bounds($table_sizes), $expected_y);
            assert_eq!(region.z_bounds($table_sizes), $expected_z);
        };
    }

    #[test]
    fn table_bounds() {
        let mc_chunk = (16, 256, 16);
        let table_size = 1024;

        // Unit case
        test_table_bounds!(
            (0.0, 0.0, 0.0),
            mc_chunk,
            table_size,
            (0, 1024),
            (0, 1024),
            (0, 1024)
        );

        // Positive
        test_table_bounds!(
            (0.0, 0.0, 0.0),
            mc_chunk,
            table_size,
            (0, 1024),
            (0, 1024),
            (0, 1024)
        );

        test_table_bounds!(
            (1925.0, 54.0, 93.0),
            mc_chunk,
            table_size,
            (1024, 2048),
            (0, 1024),
            (0, 1024)
        );

        test_table_bounds!(
            (2049.0, 54.0, 93.0),
            mc_chunk,
            table_size,
            (2048, 3072),
            (0, 1024),
            (0, 1024)
        );

        // Negative
        test_table_bounds!(
            (-0.01, -0.01, -0.01),
            mc_chunk,
            table_size,
            (-1024, 0),
            (-1024, 0),
            (-1024, 0)
        );

        test_table_bounds!(
            (-1.0, -1.0, -1.0),
            mc_chunk,
            table_size,
            (-1024, 0),
            (-1024, 0),
            (-1024, 0)
        );

        test_table_bounds!(
            (-1023.9, -1023.9, -1023.9),
            mc_chunk,
            table_size,
            (-1024, 0),
            (-1024, 0),
            (-1024, 0)
        );

        test_table_bounds!(
            (-67.0, -1025.0, -586.0),
            mc_chunk,
            table_size,
            (-1024, 0),
            (-2048, -1024),
            (-1024, 0)
        );

        // Mixed
        test_table_bounds!(
            (-45.0, 22.0, -1004.0),
            mc_chunk,
            1024,
            (-1024, 0),
            (0, 1024),
            (-1024, 0)
        );

        test_table_bounds!(
            (-45.0, 22.0, -1025.0),
            mc_chunk,
            1024,
            (-1024, 0),
            (0, 1024),
            (-2048, -1024)
        );

        test_table_bounds!(
            (-45.0, 22.0, 1015.0),
            mc_chunk,
            1024,
            (-1024, 0),
            (0, 1024),
            (0, 1024)
        );
    }
}
// endregion
