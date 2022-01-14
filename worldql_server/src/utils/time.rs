use std::num::ParseIntError;

use chrono::prelude::*;
use thiserror::Error;

pub fn parse_epoch_millis(timestamp: &str) -> Result<NaiveDateTime, ParseEpochError> {
    let ts = timestamp.parse::<u64>()?;

    let secs = (ts / 1000) as i64;
    let nsecs = ((ts % 1000) * 1_000_000) as u32;

    match NaiveDateTime::from_timestamp_opt(secs, nsecs) {
        None => Err(ParseEpochError::OutOfRangeError),
        Some(ts) => Ok(ts),
    }
}

#[derive(Debug, Error)]
pub enum ParseEpochError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    #[error("seconds out of range or invalid nanoseconds")]
    OutOfRangeError,
}
