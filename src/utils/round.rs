pub fn round_by_multiple(n: i64, multiple: i64) -> i64 {
    if multiple == 0 {
        return n;
    }

    let abs = n.abs();
    let remainder = abs % multiple;
    if remainder == 0 {
        return n;
    }

    if n < 0 {
        -(abs - remainder)
    } else {
        n + multiple - remainder
    }
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
        test_round!((0, 10), 0);
        test_round!((1, 10), 10);
        test_round!((5, 10), 10);
        test_round!((9, 10), 10);
        test_round!((9, 10), 10);
        test_round!((10, 10), 10);
        test_round!((15, 10), 20);
        test_round!((20, 10), 20);

        // Multiples of 8
        test_round!((0, 8), 0);
        test_round!((2, 8), 8);
        test_round!((5, 8), 8);
        test_round!((7, 8), 8);
        test_round!((8, 8), 8);
        test_round!((9, 8), 16);
        test_round!((15, 8), 16);
        test_round!((16, 8), 16);
    }

    #[test]
    fn round_negative() {
        // Multiples of 10
        test_round!((-1, 10), 0);
        test_round!((-5, 10), 0);
        test_round!((-9, 10), 0);
        test_round!((-9, 10), 0);
        test_round!((-10, 10), -10);
        test_round!((-15, 10), -10);
        test_round!((-20, 10), -20);

        // Multiples of 8
        test_round!((-2, 8), 0);
        test_round!((-5, 8), 0);
        test_round!((-7, 8), 0);
        test_round!((-8, 8), -8);
        test_round!((-15, 8), -8);
        test_round!((-16, 8), -16);
    }
}
