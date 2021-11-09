pub fn floor_by_multiple(n: u32, multiple: u32) -> u32 {
    if n % multiple == 0 {
        return n;
    }

    let floored = n / multiple;
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
        test_floor!((0, 10), 0);
        test_floor!((1, 10), 0);
        test_floor!((5, 10), 0);
        test_floor!((9, 10), 0);
        test_floor!((9, 10), 0);
        test_floor!((10, 10), 10);
        test_floor!((15, 10), 10);
        test_floor!((20, 10), 20);

        // Multiples of 8
        test_floor!((0, 8), 0);
        test_floor!((2, 8), 0);
        test_floor!((5, 8), 0);
        test_floor!((7, 8), 0);
        test_floor!((8, 8), 8);
        test_floor!((15, 8), 8);
        test_floor!((16, 8), 16);
    }
}
