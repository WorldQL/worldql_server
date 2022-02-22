use std::ops::RangeInclusive;

use once_cell::sync::Lazy;
use thiserror::Error;

// region: Constants
// Global World
pub const GLOBAL_WORLD: &str = "@global";

// Valid Chars
const ABC_UPPER: RangeInclusive<char> = 'A'..='Z';
const ABC_LOWER: RangeInclusive<char> = 'a'..='z';
const NUMBERS: RangeInclusive<char> = '0'..='9';
const SYMBOLS: &[char] = &['_', '-', '/', '\\', ':', '@', '#'];

// Charsets
static CHARSET: Lazy<Vec<char>> = Lazy::new(|| {
    let mut vec = vec![];

    vec.append(&mut ABC_UPPER.collect());
    vec.append(&mut ABC_LOWER.collect());
    vec.append(&mut NUMBERS.collect());
    vec.extend_from_slice(SYMBOLS);

    vec
});

// Max Length
const MAX_NAME_LENGTH: usize = 63;
// endregion

pub fn sanitize_world_name(world_name: &str) -> Option<SanitizeError> {
    if world_name == GLOBAL_WORLD {
        return Some(SanitizeError::IsGlobalWorld);
    }

    if world_name.is_empty() {
        return Some(SanitizeError::ZeroLength);
    }

    // Check for all characters being valid
    let is_valid_charset = world_name.chars().all(|char| CHARSET.contains(&char));
    if !is_valid_charset {
        return Some(SanitizeError::InvalidChars);
    }

    if world_name.len() > MAX_NAME_LENGTH {
        return Some(SanitizeError::TooLong);
    }

    None
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SanitizeError {
    #[error("is global world")]
    IsGlobalWorld,

    #[error("must be 1 or more characters long")]
    ZeroLength,

    #[error("contains invalid characters")]
    InvalidChars,

    #[error("is too long")]
    TooLong,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_sanitize_ok {
        ($input: expr) => {{
            let output = sanitize_world_name($input);
            assert!(output.is_none());
        }};
    }

    macro_rules! test_sanitize_err {
        ($input: expr, $expected: expr) => {
            let output = sanitize_world_name($input);
            let err = output.unwrap();

            assert_eq!(err, $expected);
        };
    }

    #[test]
    fn sanitize() {
        // Valid world names
        test_sanitize_ok!("world");
        test_sanitize_ok!("WORLD");
        test_sanitize_ok!("world_1_2_3");
        test_sanitize_ok!("world_one");
        test_sanitize_ok!("chat/server_1");
        test_sanitize_ok!("chat\\server_2");
        test_sanitize_ok!("chat:server_3");
        test_sanitize_ok!("chat@server_4");
        test_sanitize_ok!("chat#server_5");
        test_sanitize_ok!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");

        // Invalid global world
        test_sanitize_err!(GLOBAL_WORLD, SanitizeError::IsGlobalWorld);

        // Invalid zero length
        test_sanitize_err!("", SanitizeError::ZeroLength);

        // Invalid chars
        test_sanitize_err!("world one", SanitizeError::InvalidChars);
        test_sanitize_err!("world(two)", SanitizeError::InvalidChars);
        test_sanitize_err!("world&three", SanitizeError::InvalidChars);
        test_sanitize_err!("world*four", SanitizeError::InvalidChars);
        test_sanitize_err!("world%five", SanitizeError::InvalidChars);

        // Invalid too long
        test_sanitize_err!(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            SanitizeError::TooLong
        );
    }
}
