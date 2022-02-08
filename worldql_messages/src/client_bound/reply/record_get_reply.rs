use serde::{Deserialize, Serialize};

use crate::common::Record;

/// Replies to [`RecordGetRequest`](crate::incoming::RecordGetRequest)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordGetReply {
    /// Returned records
    pub records: Vec<Record>,
}

impl RecordGetReply {
    /// Create a new [`RecordGetReply`]
    #[inline]
    #[must_use]
    pub fn new(records: Vec<Record>) -> Self {
        Self { records }
    }

    /// Create a new [`RecordGetReply`] with no records
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        let records = vec![];
        Self { records }
    }
}

impl From<Record> for RecordGetReply {
    #[inline]
    fn from(record: Record) -> Self {
        let records = vec![record];
        Self { records }
    }
}

impl From<Vec<Record>> for RecordGetReply {
    #[inline]
    fn from(records: Vec<Record>) -> Self {
        Self { records }
    }
}
