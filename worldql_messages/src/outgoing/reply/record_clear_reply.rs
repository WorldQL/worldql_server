use serde::{Deserialize, Serialize};

/// Replies to [`crate::incoming::RecordClearRequest`]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct RecordClearReply {
    /// Number of records affected
    pub affected: u32,
}

impl RecordClearReply {
    /// Create a new [`RecordClearReply`]
    #[inline]
    #[must_use]
    pub fn new(affected: u32) -> Self {
        Self { affected }
    }
}
