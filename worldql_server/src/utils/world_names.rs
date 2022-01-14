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
const UNDERSCORE: char = '_';

// Replacements
const SPACE: (char, &str) = (' ', "_");
const FORWARD_SLASH: (char, &str) = ('/', "_fs_");
const BACK_SLASH: (char, &str) = ('\\', "_bs_");
const COLON: (char, &str) = (':', "_cl_");
const ASPERAND: (char, &str) = ('@', "_at_");

// Charsets
static CHARSET: Lazy<Vec<char>> = Lazy::new(|| {
    let mut vec = vec![];

    vec.append(&mut ABC_UPPER.collect());
    vec.append(&mut ABC_LOWER.collect());
    vec.append(&mut NUMBERS.collect());
    vec.push(UNDERSCORE);

    vec.push(SPACE.0);
    vec.push(FORWARD_SLASH.0);
    vec.push(BACK_SLASH.0);
    vec.push(COLON.0);
    vec.push(ASPERAND.0);

    vec
});

static VALID_START_CHARS: Lazy<Vec<char>> = Lazy::new(|| {
    let mut vec = vec![];

    vec.append(&mut ABC_UPPER.collect());
    vec.append(&mut ABC_LOWER.collect());

    vec
});

// Max Length
const MAX_NAME_LENGTH: usize = 63;
// endregion

pub fn sanitize_world_name(world_name: &str) -> Result<String, SanitizeError> {
    if world_name == GLOBAL_WORLD {
        return Err(SanitizeError::IsGlobalWorld);
    }

    if world_name.is_empty() {
        return Err(SanitizeError::ZeroLength);
    }

    // Check first character is a-z or A-Z
    let first_char = world_name.chars().next().unwrap();
    if !VALID_START_CHARS.contains(&first_char) {
        return Err(SanitizeError::InvalidStart);
    }

    // Check for all characters being valid
    let is_valid_charset = world_name.chars().all(|char| CHARSET.contains(&char));
    if !is_valid_charset {
        return Err(SanitizeError::InvalidChars);
    }

    // Perform replacements
    let world_name = world_name.replace(SPACE.0, SPACE.1);
    let world_name = world_name.replace(FORWARD_SLASH.0, FORWARD_SLASH.1);
    let world_name = world_name.replace(BACK_SLASH.0, BACK_SLASH.1);
    let world_name = world_name.replace(COLON.0, COLON.1);
    let world_name = world_name.replace(ASPERAND.0, ASPERAND.1);

    if world_name.len() > MAX_NAME_LENGTH {
        return Err(SanitizeError::TooLong);
    }

    Ok(world_name)
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SanitizeError {
    #[error("is global world")]
    IsGlobalWorld,

    #[error("world name must be 1 or more characters long")]
    ZeroLength,

    #[error("must start with a-z or A-Z")]
    InvalidStart,

    #[error("contains invalid characters")]
    InvalidChars,

    #[error("world name is too long")]
    TooLong,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_sanitize_ok {
        ($input: expr, $expected: expr) => {{
            let output = sanitize_world_name($input).unwrap();
            assert_eq!(output, $expected);
        }};
    }

    macro_rules! test_sanitize_err {
        ($input: expr, $expected: expr) => {
            let output = sanitize_world_name($input);
            let err = output.unwrap_err();

            assert_eq!(err, $expected);
        };
    }

    #[test]
    fn sanitize() {
        // Valid world names
        test_sanitize_ok!("world", "world");
        test_sanitize_ok!("WORLD", "WORLD");
        test_sanitize_ok!("world_1_2_3", "world_1_2_3");
        test_sanitize_ok!("world one", "world_one");
        test_sanitize_ok!("chat/server_1", "chat_fs_server_1");
        test_sanitize_ok!("chat\\server_2", "chat_bs_server_2");
        test_sanitize_ok!("chat:server_3", "chat_cl_server_3");
        test_sanitize_ok!("chat@server_4", "chat_at_server_4");
        test_sanitize_ok!(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        );

        // Invalid global world
        test_sanitize_err!(GLOBAL_WORLD, SanitizeError::IsGlobalWorld);

        // Invalid zero length
        test_sanitize_err!("", SanitizeError::ZeroLength);

        // Invalid start chars
        test_sanitize_err!("0world", SanitizeError::InvalidStart);
        test_sanitize_err!("_world", SanitizeError::InvalidStart);
        test_sanitize_err!("/world", SanitizeError::InvalidStart);
        test_sanitize_err!("\\world", SanitizeError::InvalidStart);
        test_sanitize_err!(":world", SanitizeError::InvalidStart);
        test_sanitize_err!("@world", SanitizeError::InvalidStart);
        test_sanitize_err!(" world", SanitizeError::InvalidStart);
        test_sanitize_err!("[world", SanitizeError::InvalidStart);
        test_sanitize_err!("]world", SanitizeError::InvalidStart);

        // Invalid chars
        test_sanitize_err!("world (two)", SanitizeError::InvalidChars);
        test_sanitize_err!("world&three", SanitizeError::InvalidChars);
        test_sanitize_err!("world*four", SanitizeError::InvalidChars);
        test_sanitize_err!("world-four", SanitizeError::InvalidChars);

        // Invalid too long
        test_sanitize_err!(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            SanitizeError::TooLong
        );
    }
}
