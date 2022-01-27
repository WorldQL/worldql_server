use serde::{Deserialize, Serialize};

/// Replies to [`RecordSetRequest`](crate::incoming::RecordSetRequest)
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct RecordSetReply {
    /// Number of records newly created
    created: u32,

    /// Number of existing records that were updated
    updated: u32,
}

impl RecordSetReply {
    /// Create a new [`RecordSetReply`]
    #[inline]
    #[must_use]
    pub fn new(created: u32, updated: u32) -> Self {
        Self { created, updated }
    }
}
