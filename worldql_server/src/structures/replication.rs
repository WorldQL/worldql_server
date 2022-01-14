use std::fmt::Display;

use super::{Decode, DecodeError, Encode};
use crate::flatbuffers::Replication as ReplicationFB;

#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum Replication {
    ExceptSelf,
    IncludingSelf,
    OnlySelf,
}

impl Default for Replication {
    fn default() -> Self {
        Self::ExceptSelf
    }
}

// region: Codec Traits
impl Encode<ReplicationFB> for Replication {
    #[inline]
    fn encode(self) -> ReplicationFB {
        match self {
            Replication::ExceptSelf => ReplicationFB::ExceptSelf,
            Replication::IncludingSelf => ReplicationFB::IncludingSelf,
            Replication::OnlySelf => ReplicationFB::OnlySelf,
        }
    }
}

impl Decode<ReplicationFB> for Replication {
    #[inline]
    fn decode(encoded: ReplicationFB) -> Result<Self, DecodeError> {
        let replication = match encoded {
            ReplicationFB::ExceptSelf => Replication::ExceptSelf,
            ReplicationFB::IncludingSelf => Replication::IncludingSelf,
            ReplicationFB::OnlySelf => Replication::OnlySelf,

            _ => Replication::ExceptSelf,
        };

        Ok(replication)
    }
}
// endregion

// region: Display Trait
impl Display for Replication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Replication::ExceptSelf => "ExceptSelf",
            Replication::IncludingSelf => "IncludingSelf",
            Replication::OnlySelf => "OnlySelf",
        };

        write!(f, "{}", name)
    }
}
// endregion
