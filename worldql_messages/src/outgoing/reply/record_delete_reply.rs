use serde::{Deserialize, Serialize};

/// Replies to [`crate::incoming::RecordDeleteRequest`]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct RecordDeleteReply {
    /// Number of records affected
    pub affected: u32,
}

impl RecordDeleteReply {
    /// Create a new [`RecordDeleteReply`]
    #[inline]
    #[must_use]
    pub fn new(affected: u32) -> Self {
        Self { affected }
    }
}
