use serde::{Deserialize, Serialize};

use crate::common::Record;

/// Set record(s)
///
/// Records that already exist in their region will be updated
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordSetRequest {
    records: Vec<Record>,
}

impl RecordSetRequest {
    /// Create a new [`RecordSetRequest`]
    #[inline]
    #[must_use]
    pub fn new(records: Vec<Record>) -> Self {
        Self { records }
    }
}

impl From<Record> for RecordSetRequest {
    #[inline]
    fn from(record: Record) -> Self {
        let records = vec![record];
        Self { records }
    }
}

impl From<Vec<Record>> for RecordSetRequest {
    #[inline]
    fn from(records: Vec<Record>) -> Self {
        Self { records }
    }
}
