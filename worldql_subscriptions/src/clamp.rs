/// Round an [`f64`] to the nearest multiple
///
/// Used for clamping area coordinates
pub(crate) fn round_by_multiple(n: f64, multiple: f64) -> f64 {
    if multiple == 0.0 {
        return n;
    }

    // Special case, 0 should round up
    if n == 0.0 || n == -0.0 {
        return multiple;
    }

    let ceil = (n / multiple).ceil();
    ceil * multiple
}

/// Clamp to largest absolute coordinate value
///
/// This allows us to disambiguate positive and negative areas
pub(crate) fn coord_clamp(coord: f64, size: u16) -> i64 {
    let abs_coord = coord.abs();
    let result_multiplier = match coord < 0.0 {
        true => -1,
        false => 1,
    };

    let size_i = i64::from(size);
    let size_f = f64::from(size);

    if abs_coord % size_f == 0.0 && coord != 0.0 {
        return coord as i64;
    }

    let rounded = round_by_multiple(abs_coord, f64::from(size));
    let result = match rounded > coord {
        true => rounded as i64,
        false => (rounded as i64) + size_i,
    };

    result * result_multiplier
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    // region: round_by_multiple
    macro_rules! test_round {
        ($input: expr, $expected: expr) => {
            let (n, multiple) = $input;
            let result = super::round_by_multiple(n, multiple);

            assert_eq!(result, $expected)
        };
    }

    #[test]
    fn round_positive() {
        // Multiples of 10
        test_round!((0.0, 10.0), 10.0);
        test_round!((-0.0, 10.0), 10.0);
        test_round!((0.1, 10.0), 10.0);
        test_round!((1.0, 10.0), 10.0);
        test_round!((5.0, 10.0), 10.0);
        test_round!((9.0, 10.0), 10.0);
        test_round!((9.0, 10.0), 10.0);
        test_round!((9.9999, 10.0), 10.0);
        test_round!((10.0, 10.0), 10.0);
        test_round!((10.0001, 10.0), 20.0);
        test_round!((15.0, 10.0), 20.0);
        test_round!((20.0, 10.0), 20.0);

        // Multiples of 8
        test_round!((0.0, 8.0), 8.0);
        test_round!((-0.0, 8.0), 8.0);
        test_round!((2.0, 8.0), 8.0);
        test_round!((5.0, 8.0), 8.0);
        test_round!((7.0, 8.0), 8.0);
        test_round!((8.0, 8.0), 8.0);
        test_round!((9.0, 8.0), 16.0);
        test_round!((15.0, 8.0), 16.0);
        test_round!((16.0, 8.0), 16.0);
    }

    #[test]
    fn round_negative() {
        // Multiples of 10
        test_round!((-1.0, 10.0), 0.0);
        test_round!((-5.0, 10.0), 0.0);
        test_round!((-9.0, 10.0), 0.0);
        test_round!((-9.0, 10.0), 0.0);
        test_round!((-9.9999, 10.0), 0.0);
        test_round!((-10.0, 10.0), -10.0);
        test_round!((-10.0001, 10.0), -10.0);
        test_round!((-15.0, 10.0), -10.0);
        test_round!((-20.0, 10.0), -20.0);

        // Multiples of 8
        test_round!((-2.0, 8.0), 0.0);
        test_round!((-5.0, 8.0), 0.0);
        test_round!((-7.0, 8.0), 0.0);
        test_round!((-8.0, 8.0), -8.0);
        test_round!((-15.0, 8.0), -8.0);
        test_round!((-16.0, 8.0), -16.0);
    }
    // endregion

    // region: coord_clamp
    macro_rules! test_coord_clamp {
        ($input: expr, $expected: expr) => {
            let (input, clamp) = $input;
            let actual = super::coord_clamp(input, clamp);
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
}
