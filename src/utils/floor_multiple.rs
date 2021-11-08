pub fn floor_by_multiple(n: f64, multiple: f64) -> f64 {
    if n % multiple == 0.0 {
        return n
    }

    let floored = (n / multiple).floor();
    floored * multiple
}

#[cfg(test)]
mod tests {
    macro_rules! test_floor {
        ($input: expr, $expected: expr) => {
            let (n, multiple) = $input;
            let result = super::floor_by_multiple(n, multiple);

            assert_eq!(result, $expected)
        };
    }

    #[test]
    fn floor_by_multiple() {
        // Multiples of 10
        test_floor!((0.0, 10.0), 0.0);
        test_floor!((0.5, 10.0), 0.0);
        test_floor!((1.4, 10.0), 0.0);
        test_floor!((5.8, 10.0), 0.0);
        test_floor!((9.9, 10.0), 0.0);
        test_floor!((9.999999, 10.0), 0.0);
        test_floor!((10.0, 10.0), 10.0);
        test_floor!((15.0, 10.0), 10.0);
        test_floor!((20.0, 10.0), 20.0);

        // Multiples of 8
        test_floor!((0.0, 8.0), 0.0);
        test_floor!((0.7, 8.0), 0.0);
        test_floor!((2.2, 8.0), 0.0);
        test_floor!((6.7, 8.0), 0.0);
        test_floor!((8.0, 8.0), 8.0);
        test_floor!((15.999999, 8.0), 8.0);
        test_floor!((16.0, 8.0), 16.0);
    }
}
