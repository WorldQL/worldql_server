pub fn round_by_multiple(n: f64, multiple: f64) -> f64 {
    if multiple == 0.0 {
        return n;
    }
    // special case, 0 rounds to 10.
    if n == 0.0 {
        return multiple;
    }

    let ceil = (n / multiple).ceil();
    ceil * multiple
}

#[cfg(test)]
mod tests {
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
}
