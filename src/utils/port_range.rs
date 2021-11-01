use std::fmt::Debug;
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

use thiserror::Error;

pub struct PortRange(RangeInclusive<u16>);

impl PortRange {
    pub fn inner(&self) -> RangeInclusive<u16> {
        self.0.clone()
    }
}

impl From<PortRange> for RangeInclusive<u16> {
    fn from(port_range: PortRange) -> Self {
        port_range.0
    }
}

impl FromStr for PortRange {
    type Err = PortRangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split("..").collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(PortRangeParseError::Invalid);
        }

        let first = split.get(0).unwrap().parse::<u16>()?;
        let second = split.get(1).unwrap().parse::<u16>()?;

        if first == second {
            return Err(PortRangeParseError::EqualError);
        }

        if second < first {
            return Err(PortRangeParseError::OrderError);
        }

        let port_range = Self(first..=second);
        Ok(port_range)
    }
}

#[derive(Debug, Error)]
pub enum PortRangeParseError {
    #[error("invalid port range")]
    Invalid,

    #[error("start and end ports cannot be equal")]
    EqualError,

    #[error("start port must be greater than end port")]
    OrderError,

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

impl Debug for PortRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
